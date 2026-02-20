use crate::{
    compiler::route::Route,
    indexer::{FileSlot, Slots},
};
use anyhow::Result;
use mime_guess::mime;
use std::{io, path::Path, sync::Arc};
use typst::syntax::{FileId, VirtualPath};

pub fn compile(slot: &FileSlot, slots: &Arc<Slots>) -> Result<Route> {
    let fs = GrassSlotsFs(slots);
    let opts = grass::Options::default()
        .style(grass::OutputStyle::Compressed)
        .input_syntax(grass::InputSyntax::Scss)
        .fs(&fs);
    let path = slot.id.vpath().as_rooted_path();
    let content = grass::from_path(path, &opts)?;
    Ok(Route::from_string(content, mime::TEXT_CSS, None))
}

/// Make `Slots` VFS for Grass
#[derive(Debug)]
struct GrassSlotsFs<'a>(&'a Arc<Slots>);

impl<'a> grass::Fs for GrassSlotsFs<'a> {
    fn is_dir(&self, _: &Path) -> bool {
        false
    }

    fn is_file(&self, path: &Path) -> bool {
        self.slot(path).is_some()
    }

    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        match self.slot(path) {
            Some(slot) => Ok(slot.file.to_vec()),
            None => Err(io::Error::from_raw_os_error(libc::ENOENT)),
        }
    }
}

impl<'a> GrassSlotsFs<'a> {
    fn slot(&self, path: &Path) -> Option<&FileSlot> {
        let vp = VirtualPath::new(path);
        let id = FileId::new(None, vp);
        self.0.get(&id)
    }
}
