use clap::Parser;
use std::fs;

mod init;

#[derive(Parser)]
struct Cli {
    function: String,
    template: String,
}

const template_dir: &str = "~/.tf-templates/";

fn main() {
    let args = Cli::parse();

    let current_templates = init::init_app().except("Failed to load templates");



}
