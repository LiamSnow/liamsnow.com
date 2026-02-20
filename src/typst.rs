//! This is basically a reimplementation of [typst-cli](https://docs.rs/crate/typst-cli/0.14.2/source/src/)
//! for my specific use case only.
//!
//! A big modification/optimization here is there is no lazy loading of files.
//! The program will need all files read at some point, so it async reads
//! all files in the index, then all are shared here

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use rustc_hash::FxHashMap;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};
use typst::comemo::Track;
use typst::diag::{
    FileError, FileResult, HintedStrResult, Severity, SourceDiagnostic, Warned, bail,
};
use typst::ecow::{EcoString, eco_format};
use typst::engine::Sink;
use typst::foundations::{Bytes, Datetime, Dict, LocatableSelector, Scope, Value};
use typst::syntax::{FileId, Lines, Source, Span, SyntaxMode};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Feature, Library, LibraryExt, World, WorldExt};
use typst_eval::eval_string;
use typst_html::HtmlDocument;

use crate::CONFIG;
use crate::indexer::FileSlot;

static BOOK: LazyLock<LazyHash<FontBook>> = LazyLock::new(|| LazyHash::new(FontBook::default()));
static WORKDIR: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap());
static STDLIB: LazyLock<LazyHash<Library>> = LazyLock::new(|| {
    LazyHash::new(
        Library::builder()
            .with_features([Feature::Html].into_iter().collect())
            .build(),
    )
});

pub struct LiamsWorld {
    /// The root relative to which absolute paths are resolved.
    // root: PathBuf,
    /// The input path.
    main: FileId,
    /// Typst's standard library.
    library: Option<LazyHash<Library>>,
    /// Maps file ids to source files and buffers.
    slots: Arc<FxHashMap<FileId, FileSlot>>,
}

pub fn library(inputs: Dict) -> LazyHash<Library> {
    LazyHash::new(
        Library::builder()
            .with_features([Feature::Html].into_iter().collect())
            .with_inputs(inputs)
            .build(),
    )
}

impl LiamsWorld {
    pub fn new(
        main: FileId,
        slots: Arc<FxHashMap<FileId, FileSlot>>,
        library: Option<LazyHash<Library>>,
    ) -> Self {
        Self {
            main,
            library,
            slots,
        }
    }

    /// Compile the document
    pub fn compile(&mut self) -> anyhow::Result<HtmlDocument> {
        let Warned { output, warnings } = typst::compile::<HtmlDocument>(self);

        match output {
            Ok(doc) => {
                self.print_diagnostics(&[], &warnings)?;
                Ok(doc)
            }
            Err(errors) => {
                self.print_diagnostics(&errors, &warnings)?;
                anyhow::bail!("compilation failed")
            }
        }
    }

    /// Generate HTML output
    pub fn html(&self, doc: &HtmlDocument) -> anyhow::Result<String> {
        let mut html = match typst_html::html(doc) {
            Ok(c) => c,
            Err(errors) => {
                self.print_diagnostics(&errors, &[])?;
                anyhow::bail!("html output failed")
            }
        };

        let cfg = CONFIG.get().unwrap();

        if cfg.watch {
            html = html.replacen("</head>", &format!(r#"
            <script>
                (function() {{                                                                              
                    const ws = new WebSocket(`ws://{}:{}`);                              
                    ws.onmessage = () => location.reload();                                                  
                    ws.onclose = () => setTimeout(() => location.reload(), 1000);                            
                }})();
            </script>
            </head>
            "#, cfg.watch_address, cfg.watch_port), 1);
        }

        Ok(html)
    }

    /// Query the document
    /// Must be called after `.compile`
    /// This is effectively equivalent to:
    /// ```bash
    /// typst query <selector> --field value --one
    /// ```
    /// This function only fails from internal issues
    pub fn query(&self, doc: &HtmlDocument, selector: &str) -> HintedStrResult<Value> {
        let selector = eval_string(
            &typst::ROUTINES,
            Track::track(self),
            // TODO: propagate warnings
            Sink::new().track_mut(),
            selector,
            Span::detached(),
            SyntaxMode::Code,
            Scope::default(),
        )
        .map_err(|errors| {
            let mut message = EcoString::from("failed to evaluate selector");
            for (i, error) in errors.into_iter().enumerate() {
                message.push_str(if i == 0 { ": " } else { ", " });
                message.push_str(&error.message);
            }
            message
        })?
        .cast::<LocatableSelector>()?;

        let data = doc
            .introspector
            .query(&selector.0)
            .into_iter()
            .filter_map(|c| c.get_by_name("value").ok())
            .collect::<Vec<_>>();

        if data.is_empty() {
            bail!("query returned no results");
        }

        if data.len() > 1 {
            bail!("query returned too many results ({})", data.len());
        }

        Ok(data[0].clone())
    }
}

impl World for LiamsWorld {
    fn library(&self) -> &LazyHash<Library> {
        self.library.as_ref().unwrap_or(&STDLIB)
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &BOOK
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        let slot = self.slots.get(&id).ok_or(FileError::AccessDenied)?;
        let source = slot.source.as_ref().ok_or(FileError::NotSource)?;
        Ok(source.clone())
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let slot = self.slots.get(&id).ok_or(FileError::AccessDenied)?;
        Ok(slot.file.clone())
    }

    fn font(&self, _: usize) -> Option<Font> {
        None
    }

    fn today(&self, _: Option<i64>) -> Option<Datetime> {
        None
    }
}

impl LiamsWorld {
    pub fn print_diagnostics(
        &self,
        errors: &[SourceDiagnostic],
        warnings: &[SourceDiagnostic],
    ) -> Result<(), codespan_reporting::files::Error> {
        let writer = StandardStream::stderr(ColorChoice::Auto);
        let config = codespan_reporting::term::Config::default();

        for diagnostic in warnings.iter().chain(errors) {
            if diagnostic
                .message
                .contains("html export is under active development")
            {
                // dammit bruh
                continue;
            }

            let diag = match diagnostic.severity {
                Severity::Error => Diagnostic::error(),
                Severity::Warning => Diagnostic::warning(),
            }
            .with_message(diagnostic.message.clone())
            .with_notes(
                diagnostic
                    .hints
                    .iter()
                    .map(|e| (eco_format!("hint: {e}")).into())
                    .collect(),
            )
            .with_labels(self.label(diagnostic.span).into_iter().collect());

            term::emit(&mut writer.lock(), &config, self, &diag)?;

            // Stacktrace-like helper diagnostics.
            for point in &diagnostic.trace {
                let message = point.v.to_string();
                let help = Diagnostic::help()
                    .with_message(message)
                    .with_labels(self.label(point.span).into_iter().collect());

                term::emit(&mut writer.lock(), &config, self, &help)?;
            }
        }
        Ok(())
    }

    fn label(&self, span: Span) -> Option<Label<FileId>> {
        Some(Label::primary(span.id()?, self.range(span)?))
    }

    /// Lookup line metadata for a file by id.
    pub fn lookup(&self, id: FileId) -> CodespanResult<Lines<String>> {
        let slot = self.slots.get(&id).ok_or(CodespanError::FileMissing)?;

        if let Some(source) = &slot.source {
            Ok(source.lines().clone())
        } else {
            Ok(Lines::new(String::from_utf8_lossy(&slot.file).to_string()))
        }
    }
}

type CodespanResult<T> = Result<T, CodespanError>;
type CodespanError = codespan_reporting::files::Error;

impl<'a> codespan_reporting::files::Files<'a> for LiamsWorld {
    type FileId = FileId;
    type Name = String;
    type Source = Lines<String>;

    fn name(&'a self, id: FileId) -> CodespanResult<Self::Name> {
        let vpath = id.vpath();
        Ok(if let Some(package) = id.package() {
            format!("{package}{}", vpath.as_rooted_path().display())
        } else {
            // Try to express the path relative to the working directory.
            vpath
                .resolve(&CONFIG.get().unwrap().root)
                .and_then(|abs| pathdiff::diff_paths(abs, &*WORKDIR))
                .as_deref()
                .unwrap_or_else(|| vpath.as_rootless_path())
                .to_string_lossy()
                .into()
        })
    }

    fn source(&'a self, id: FileId) -> CodespanResult<Self::Source> {
        self.lookup(id)
    }

    fn line_index(&'a self, id: FileId, given: usize) -> CodespanResult<usize> {
        let source = self.lookup(id)?;
        source
            .byte_to_line(given)
            .ok_or_else(|| CodespanError::IndexTooLarge {
                given,
                max: source.len_bytes(),
            })
    }

    fn line_range(&'a self, id: FileId, given: usize) -> CodespanResult<std::ops::Range<usize>> {
        let source = self.lookup(id)?;
        source
            .line_to_range(given)
            .ok_or_else(|| CodespanError::LineTooLarge {
                given,
                max: source.len_lines(),
            })
    }

    fn column_number(&'a self, id: FileId, _: usize, given: usize) -> CodespanResult<usize> {
        let source = self.lookup(id)?;
        source.byte_to_column(given).ok_or_else(|| {
            let max = source.len_bytes();
            if given <= max {
                CodespanError::InvalidCharBoundary { given }
            } else {
                CodespanError::IndexTooLarge { given, max }
            }
        })
    }
}
