use std::{env, path::PathBuf};

use clap::{Args, Parser, Subcommand};

mod init;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Generate(Generate)
}

#[derive(Args)]
struct Generate {
    #[arg(short, long, help = "Name of the template you want to use (based on folder name of the template ~/.tfstarter/)")]
    template: Option<String>
}

fn main() {
    let cli = Cli::parse();

    let _templates = init::init_app().expect("Failed to load templates");

    match &cli.cmd {
        Some(Commands::Generate(template)) => {
            match template.template {
                Some(ref _template) => {
                    let home_dir = env::var("HOME").unwrap();
                    let mut src = PathBuf::from(home_dir);
                    src.push(".tfstarter/");
                    src.push(_template);
                    let dst = env::current_dir().unwrap();
                    utils::copy_dir_all(src, dst).unwrap();
                    println!("{}", _template);
                }
                None => {
                    println!("Error : missing template ref. See tfstarter generate -h");
                }
            }
        }
        None => {}
    }

}
