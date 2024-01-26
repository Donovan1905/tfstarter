use std::fs;
use csv::Writer;
use std::path::Path;
use std::env;
use std::io;


pub fn init_app() -> Result<Vec<_>, Box<dyn std::error::Error>> {

    let home_dir = env::var("HOME")?;
    let config_folder_path = Path::new(&home_dir).join(".tfstarter/");

    // Create config directory if not exists
    fs::create_dir_all(config_folder_path.clone())?;


    let mut entries = fs::read_dir(config_folder_path.clone())?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}