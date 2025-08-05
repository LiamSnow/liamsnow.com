#[cfg(feature = "dev")]
use {
    notify::{RecommendedWatcher, RecursiveMode, Watcher},
    std::{fs, path::Path},
    tokio::sync::mpsc,
};

#[cfg(not(feature = "dev"))]
use grass::OutputStyle;

#[cfg(not(feature = "dev"))]
pub fn compile_file(base_dir: &str, name: &str) -> String {
    let opts = grass::Options::default().style(OutputStyle::Compressed);
    grass::from_path(format!("{base_dir}/static/{name}.scss"), &opts).unwrap()
}

#[cfg(feature = "dev")]
pub fn watch(base_dir: String) {
    fs::create_dir_all(format!("{base_dir}/static/dist")).ok();
    compile_all_scss_files(&base_dir);

    tokio::spawn(async move {
        let (tx, mut rx) = mpsc::channel(100);

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    tx.blocking_send(event).ok();
                }
            },
            notify::Config::default(),
        )
        .unwrap();

        watcher
            .watch(
                Path::new(&format!("{base_dir}/static")),
                RecursiveMode::NonRecursive,
            )
            .unwrap();

        while let Some(event) = rx.recv().await {
            use notify::EventKind;

            let is_modify = matches!(
                event.kind,
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
            );
            let has_scss = event
                .paths
                .iter()
                .any(|p| p.extension().and_then(|s| s.to_str()) == Some("scss"));

            if is_modify && has_scss {
                compile_all_scss_files(&base_dir);
            }
        }
    });
}

#[cfg(feature = "dev")]
fn compile_all_scss_files(base_dir: &str) {
    println!("Recompiling scss..");

    let Ok(entries) = fs::read_dir("static") else {
        eprintln!("Failed to read static directory");
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();

        let Some("scss") = path.extension().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };

        if name.starts_with('_') {
            continue;
        }

        match grass::from_path(&path, &grass::Options::default()) {
            Ok(css) => {
                let css_name = name.replace(".scss", ".css");
                let output_path = format!("{base_dir}/static/dist/{css_name}");

                if let Err(e) = fs::write(&output_path, css) {
                    eprintln!("Failed to write {output_path}: {e}");
                } else {
                    println!("Compiled {name} -> {output_path}");
                }
            }
            Err(e) => eprintln!("Failed to compile {name}: {e}"),
        }
    }
}
