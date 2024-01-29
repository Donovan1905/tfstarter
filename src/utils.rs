use std::fs::{copy, create_dir_all, read_dir, read_to_string, write};
use std::path::{Path, PathBuf};

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
