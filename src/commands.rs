use std::{env, path::Path};

use crate::utils;


pub fn generate(template: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let dst = env::current_dir().unwrap();
    utils::copy_dir_all(template, dst).unwrap();
    Ok(())
}