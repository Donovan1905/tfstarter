use std::{env, fs::create_dir_all, path::Path};

extern crate reqwest;

use crate::utils;

pub fn init_app() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let config_folder_path = Path::new(&home_dir).join(".tfstarter/");

    if !config_folder_path.exists() {
        create_dir_all(config_folder_path.clone())?;
        utils::update_default_templates().expect("Failed to load default templates");
        Ok(())
    } else {
        Ok(())
    }
}
