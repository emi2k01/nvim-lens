use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) plugins: Vec<Plugin>,
}

#[derive(Deserialize)]
pub(crate) struct Plugin {
    pub(crate) id: String,
    pub(crate) url: String,
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) colorschemes: Vec<String>,
}

pub(crate) fn load(path: &Path) -> Config {
    let file = std::fs::File::open(path).unwrap();
    return serde_json::from_reader(file).unwrap();
}
