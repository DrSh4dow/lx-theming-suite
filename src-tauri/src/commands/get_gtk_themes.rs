use std::{fs, path::Path};
use tauri::State;

use crate::state::{Compatibility, GTKTheme, GTKThemeStorage};

#[tauri::command]
pub fn get_gtk_themes(theme_storage: State<GTKThemeStorage>) -> Vec<GTKTheme> {
    tracing::debug!("Walking through directories");
    for entry in vec![
        Path::new("/usr/share/themes"),
        Path::new("~/.local/share/themes"),
        Path::new("~/.themes"),
    ]
    .into_iter()
    .filter_map(|path| fs::read_dir(path).ok())
    .flatten()
    .filter_map(|p| p.ok())
    {
        let mut theme_storage = match theme_storage.themes.lock() {
            Ok(themes)
                if !themes.contains_key(&entry.file_name().to_string_lossy().to_string()) =>
            {
                themes
            }
            _ => continue,
        };

        let file_name = entry.file_name().to_string_lossy().to_string();

        let theme = GTKTheme {
            name: entry.file_name().to_string_lossy().to_string(),
            description: match std::fs::read_to_string(entry.path().join("index.theme")) {
                Ok(content) => match content.lines().find(|l| l.starts_with("Comment=")) {
                    Some(comment) => comment.split("Comment=").last().map(|desc| desc.to_owned()),
                    None => None,
                },
                Err(_) => None,
            },
            compatibility: Compatibility {
                gtk2: entry.path().join("gtk-2.0/gtkrc").exists(),
                gtk3: entry.path().join("gtk-3.0/gtk.css").exists(),
                gtk4: entry.path().join("gtk-4.0/gtk.css").exists(),
            },
            raw_entry: Some(entry),
        };

        if !theme.compatibility.gtk2 && !theme.compatibility.gtk3 && !theme.compatibility.gtk4 {
            continue;
        }

        theme_storage.insert(file_name, theme);
    }

    match theme_storage.themes.lock() {
        Ok(themes) => themes
            .values()
            .map(|v| GTKTheme {
                name: v.name.clone(),
                description: v.description.clone(),
                compatibility: v.compatibility,
                raw_entry: None,
            })
            .collect(),

        Err(_) => Vec::new(),
    }
}
