use std::{collections::HashMap, fs::DirEntry, sync::Mutex};

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone, Copy)]
pub struct Compatibility {
    pub gtk2: bool,
    pub gtk3: bool,
    pub gtk4: bool,
}

#[derive(Serialize, Debug)]
pub struct GTKTheme {
    pub name: String,
    pub description: Option<String>,
    pub compatibility: Compatibility,

    #[serde(skip_serializing)]
    pub raw_entry: Option<DirEntry>,
}

pub struct GTKThemeStorage {
    pub themes: Mutex<HashMap<String, GTKTheme>>,
}
