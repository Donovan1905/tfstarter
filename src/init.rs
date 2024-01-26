use std::{fs,path::{Path, PathBuf},env,io};

pub fn init_app() -> Result<(), Box<dyn std::error::Error>> {

    let home_dir = env::var("HOME")?;
    let config_folder_path = Path::new(&home_dir).join(".tfstarter/");

    // Create config directory if not exists
    fs::create_dir_all(config_folder_path.clone())?;

    Ok(())
}