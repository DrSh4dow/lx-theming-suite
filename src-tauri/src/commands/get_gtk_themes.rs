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
    let themes = vec![
        Theme {
            name: "Adwaita".to_string(),
            description: "The default GNOME theme".to_string(),
            preview: "/test".to_string(),
            path: "src/theme/adwaita.test".to_string(),
        },
        Theme {
            name: "Adwaita-dark".to_string(),
            description: "The default GNOME dark theme".to_string(),
            preview: "/test".to_string(),
            path: "src/theme/adwaita-dark.test".to_string(),
        },
        Theme {
            name: "Adwaita-highcontrast".to_string(),
            description: "The default GNOME high contrast theme".to_string(),
            preview: "/test".to_string(),
            path: "src/theme/adwaita-highcontrast.test".to_string(),
        },
        Theme {
            name: "Adwaita-highcontrast-inverted".to_string(),
            description: "The default GNOME high contrast inverted theme".to_string(),
            preview: "/test".to_string(),
            path: "src/theme/adwaita-highcontrast-inverted.test".to_string(),
        },
        Theme {
            preview: "/test".to_string(),
            name: "Gruvbox".to_string(),
            description: "A retro groove color scheme".to_string(),
            path: "src/theme/gruvbox.test".to_string(),
        },
        Theme {
            preview: "/test".to_string(),
            name: "Tokio Night".to_string(),
            description: "A dark theme for Visual Studio Code and 50+ apps".to_string(),
            path: "src/theme/tokio-night.test".to_string(),
        },
    ];

    themes
}
