use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

const ROUTES_FILE: &str = "routes.toml";
pub const CONTENT_DIR: &str = "./content";

#[derive(Debug, Deserialize)]
struct Config {
    routes: Vec<ConfigRoute>,
}

#[derive(Debug, Deserialize)]
struct ConfigRoute {
    path: String,
    file: Option<String>,
    /// nest directory using another routes.toml file
    nest_dir: Option<String>,
    /// nest all files in directory
    auto_nest_dir: Option<String>,
}

#[derive(Clone)]
pub struct FileTask {
    pub url_path: String,
    pub file_path: PathBuf,
}

pub fn load(base_path: &str, dir: &PathBuf) -> Result<Vec<FileTask>> {
    let config_path = dir.join(ROUTES_FILE);
    let config_str = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read {}", config_path.display()))?;
    let config: Config = toml::from_str(&config_str)?;
    let mut tasks = Vec::with_capacity(config.routes.len());

    for route in config.routes {
        let path = if base_path.is_empty() {
            route.path.clone()
        } else {
            format!("{}{}", base_path, route.path)
        };

        // direct file
        if let Some(file) = route.file {
            tasks.push(FileTask {
                url_path: path.clone(),
                file_path: dir.join(&file),
            });
        }

        // nest dir using another routes.toml file
        if let Some(subdir) = route.nest_dir {
            let subdir_path = dir.join(&subdir);
            let nested_tasks = load(&path, &subdir_path)?;
            tasks.extend(nested_tasks);
        }

        // nest all files in dir
        if let Some(subdir) = route.auto_nest_dir {
            let subdir_path = dir.join(&subdir);

            for entry in WalkDir::new(&subdir_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if !entry.file_type().is_file() {
                    continue;
                }

                let file_path = entry.path();
                let relative = file_path.strip_prefix(&subdir_path)?;
                let url_segment = relative.to_str().context("Invalid UTF-8 in path")?;

                let full_path = format!("{}/{}", path.trim_end_matches('/'), url_segment);
                tasks.push(FileTask {
                    url_path: full_path,
                    file_path: file_path.to_path_buf(),
                });
            }
        }
    }

    Ok(tasks)
}
