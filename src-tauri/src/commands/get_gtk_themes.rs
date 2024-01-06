use itertools::Itertools;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize, Default, Debug)]
pub struct Compatibility {
    gtk2: bool,
    gtk3: bool,
    gtk4: bool,
}

#[derive(Serialize, Debug)]
pub struct Theme {
    name: String,
    description: String,
    path: String,
    compatibility: Compatibility,

    #[serde(skip_serializing)]
    raw_entry: walkdir::DirEntry,
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
            compatibility: Compatibility::default(),
            raw_entry: entry,
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
    .map(|mut t| {
        if let Some(parent) = t.raw_entry.path().parent() {
            t.compatibility.gtk2 = parent.join("gtk-2.0/gtkrc").exists();
            t.compatibility.gtk3 = parent.join("gtk-3.0/gtk.css").exists();
            t.compatibility.gtk4 = parent.join("gtk-4.0/gtk.css").exists();

            tracing::debug!("{:?}", parent.to_string_lossy().to_string());
        }

        t
    })
    .collect();

    themes
}
