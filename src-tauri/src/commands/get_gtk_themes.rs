use serde::Serialize;

#[derive(Serialize)]
pub struct Theme {
    name: String,
    description: String,
    preview: String,
    path: String,
}

#[tauri::command]
pub fn get_gtk_themes() -> Vec<Theme> {
    tracing::info!("Walking through directories");

    if std::path::Path::new("/usr/share/themes").exists() {
        for entry in walkdir::WalkDir::new("/usr/share/themes")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().eq("index.theme"))
        {
            println!("{}", entry.path().display());
        }
    }

    Vec::new()
}
