use regex::Regex;
use std::collections::HashSet;
use std::{env, fs::{File, create_dir_all, copy, remove_file, read_dir, read_to_string, write}, path::{Path}, io};
use zip::ZipArchive;


pub fn copy_dir_all(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(&dst).expect("Failed to create dir");
    for entry in read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn replace_tag_with_string(
    src: impl AsRef<Path>,
    tag: String,
    replace_with: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let template = read_to_string(src.as_ref()).expect("Unable to read template file");

    let full_tag = "{{".to_string() + &tag + "}}";
    let result = template.replace(&full_tag, &replace_with);

    write(src, result).expect("Unable to write file");

    Ok(())
}

pub fn list_tags_in_template(
    src: impl AsRef<Path>,
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let mut all_placeholders: HashSet<String> = HashSet::new();
    let re = Regex::new(r"\{\{(.+?)\}\}")?;
    for entry in read_dir(src.as_ref())? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            all_placeholders = list_tags_in_template(entry.path())?;
        } else {
            let template = read_to_string(entry.path()).expect("Unable to read template file");

            let placeholders: HashSet<String> = re
                .captures_iter(&template)
                .filter_map(|cap| cap.get(1))
                .map(|match_| match_.as_str().to_string())
                .collect();

            all_placeholders.extend(placeholders);
        }
    }
    Ok(all_placeholders)
}

pub fn replace_tag_with_string_all(
    src: impl AsRef<Path>,
    tag: String,
    replace_with: String,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            replace_tag_with_string_all(entry.path(), tag.clone(), replace_with.clone())?;
        } else {
            replace_tag_with_string(entry.path(), tag.clone(), replace_with.clone())?;
        }
    }
    Ok(())
}

pub fn update_default_templates() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let config_folder_path = Path::new(&home_dir).join(".tfstarter/");
    let template_download_url = env::var("S3_TEMPLATE_URL").expect("Failed to get S3_TEMPLATE_URL var");

    let mut response = reqwest::blocking::get(template_download_url)?;

    let zip_path = Path::new("./templates.zip");

    let mut out = File::create(zip_path).expect("failed to create file");
    io::copy(&mut response, &mut out).expect("failed to copy content");

    extract_zip(zip_path, config_folder_path.as_path().as_ref()).unwrap();

    remove_file(zip_path).unwrap();

    Ok(())
}

fn extract_zip(archive_path: &Path, destination: &Path) -> io::Result<()> {
    // Open the zip file
    let file = File::open(archive_path)?;
    let mut zip = ZipArchive::new(file)?;

    // Iterate over each file & directory in the archive
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = destination.join(file.mangled_name());

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

