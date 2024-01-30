use std::{env, fs::{self, File, create_dir_all, copy}, path::{Path, PathBuf}, io};
use std::io::Read;
use zip::ZipArchive;

extern crate reqwest;
use tempfile::Builder;

pub fn init_app() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let config_folder_path = Path::new(&home_dir).join(".tfstarter/");

    // Create config directory if not exists
    create_dir_all(config_folder_path.clone())?;

    update_default_templates();

    Ok(())
}

fn update_default_templates() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let config_folder_path = Path::new(&home_dir).join(".tfstarter/");

    let target = "https://tfstarter-templates-bucket.s3.eu-west-3.amazonaws.com/templates.zip";

    let mut response = reqwest::blocking::get(target)?;

    let mut out = File::create("./templates.zip").expect("failed to create file");
    io::copy(&mut response, &mut out).expect("failed to copy content");

    let zip_path = Path::new("./templates.zip");
    let extract_to = Path::new("./test");

    extract_zip(zip_path, extract_to).unwrap();

    Ok(())
}

fn extract_zip(archive_path: &Path, destination: &Path) -> io::Result<()> {
    // Open the zip file
    let file = File::open(archive_path)?;
    let mut zip = ZipArchive::new(file)?;

    // Iterate over each file & directory in the archive
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = destination.join(file.sanitized_name());

        if file.name().ends_with('/') {
            // Create a directory
            std::fs::create_dir_all(&outpath)?;
        } else {
            // Ensure the parent directory exists
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }

            // Write file
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
