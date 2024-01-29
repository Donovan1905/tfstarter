use std::{env, fs::{self, File, create_dir_all}, path::{Path, PathBuf}};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubFile {
    name: String,
    download_url: String,
}

pub fn init_app() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let config_folder_path = Path::new(&home_dir).join(".tfstarter/");

    // Create config directory if not exists
    fs::create_dir_all(config_folder_path.clone())?;

    Ok(())
}

pub fn update_default_templates() -> Result<(), Box<dyn std::error::Error>> {


    Ok(())
}
