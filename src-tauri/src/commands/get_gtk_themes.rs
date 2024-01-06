use itertools::Itertools;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize, Debug)]
pub struct Theme {
    name: String,
    description: String,
    path: String,
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[tauri::command]
pub fn get_gtk_themes() -> Vec<Theme> {
    tracing::debug!("Walking through directories");

    let themes: Vec<Theme> = vec![
        Path::new("/usr/share/themes"),
        Path::new("~/.local/share/themes"),
        Path::new("~/.themes"),
    ]
    .into_iter()
    .filter_map(|path| match path.exists() {
        true => Some(path.to_string_lossy().to_string()),
        false => None,
    })
    .flat_map(|path| walkdir::WalkDir::new(path).follow_links(true).into_iter())
    .filter_map(|e| e.ok())
    .filter(|e| {
        e.file_name().to_string_lossy().eq("index.theme") && e.metadata().is_ok_and(|m| m.is_file())
    })
    .filter_map(|entry| {
        let file_content = match std::fs::read_to_string(entry.path()) {
            Ok(content) => content,
            Err(_) => return None,
        };

        let mut theme = Theme {
            name: String::from(""),
            description: String::from(""),
            path: entry.path().to_string_lossy().to_string(),
        };

        for line in file_content.lines() {
            if line.starts_with("Name=") {
                if let Some(name) = line.split("Name=").last() {
                    theme.name = String::from(name);
                };
            }

            if line.starts_with("Comment=") {
                if let Some(comment) = line.split("Comment=").last() {
                    theme.description = String::from(comment);
                };
            }
        }

        Some(theme)
    })
    .unique_by(|t| t.name.clone())
    .collect();

    themes
}
