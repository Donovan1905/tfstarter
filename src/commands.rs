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

pub fn replace(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let dst = env::current_dir().unwrap();
    utils::copy_dir_all(path.as_ref(), dst.clone()).unwrap();

    let placeholders = utils::list_tags_in_template(dst.clone()).unwrap();

    for placeholder in placeholders {
        let mut input = String::new();
        println!(
            "Provide a value for {} : ",
            placeholder.clone().bold().bright_purple()
        );
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                utils::replace_tag_with_string_all(dst.clone(), placeholder.clone(), input.clone())
                    .unwrap();
            }
            Err(error) => println!("Failed to get user input. Error : {error}"),
        }
    }
    Ok(())
}
