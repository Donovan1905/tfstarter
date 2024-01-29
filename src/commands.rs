use colored::Colorize;
use std::{env, fs::read_dir, io, path::Path};

use crate::utils;

pub fn generate(template: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let dst = env::current_dir().unwrap();
    utils::copy_dir_all(template, dst).unwrap();
    Ok(())
}

pub fn get(template_dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let entries = read_dir(template_dir)
        .expect("Error : Failed to read templates folder")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    for tpl in &entries {
        println!(
            "> {}",
            tpl.into_iter()
                .last()
                .unwrap()
                .to_str()
                .unwrap()
                .bold()
                .purple()
        )
    }

    Ok(())
}

pub fn replace(
    path: impl AsRef<Path>,
    placeholder: String,
    replace_with: String,
) -> Result<(), Box<dyn std::error::Error>> {
    utils::replace_tag_with_string(path, placeholder.clone(), replace_with.clone()).unwrap();

    Ok(())
}
