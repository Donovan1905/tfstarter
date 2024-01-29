use std::collections::HashSet;
use std::fs::{copy, create_dir_all, read_dir, read_to_string, write};
use std::hash::Hash;
use std::path::{Path};
use regex::Regex;

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

pub fn list_tags_in_template (
    src: impl AsRef<Path>
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

            let placeholders: HashSet<String> = re.captures_iter(&template)
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
