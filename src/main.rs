use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use std::{env, path::PathBuf};

mod commands;
mod init;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Get(Get),
    Generate(Generate),
    Replace(Replace),
}

#[derive(Args)]
struct Generate {
    #[arg(
        short,
        long,
        help = "Name of the template you want to use (based on folder name of the template ~/.tfstarter/)"
    )]
    template: Option<String>,
}

#[derive(Args)]
struct Get {
    #[arg(short, long, help = "Filter by tag (coming soon)")]
    #[clap(default_value = "A")]
    ttype: Option<String>,
}

#[derive(Args)]
struct Replace {
    #[arg(short, long)]
    template: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    init::init_app().expect("Failed to load templates");

    let home_dir = env::var("HOME").unwrap();
    let mut src = PathBuf::from(home_dir);
    src.push(".tfstarter/");

    match &cli.cmd {
        Some(Commands::Generate(template)) => match template.template {
            Some(ref _template) => {
                src.push(_template);
                commands::generate(src).expect("Failed to generate template");
                println!("Template {} copied in current directory", _template);
            }
            None => {
                println!("Error : missing template ref. See tfstarter generate -h");
            }
        },
        Some(Commands::Get(ttype)) => match ttype.ttype {
            Some(ref _ttype) => {
                commands::get(src).expect("Failed to list templates");
            }
            None => {
                println!("Error : missing type")
            }
        },
        Some(Commands::Replace(arg)) => match arg.template {
            Some(ref _template) => {
                src.push(_template);
                commands::replace(src).expect("Failed to replace placeholder in template");
                println!(
                    "\nReplaced placeholder in template {}",
                    _template.bold().green()
                );
            }
            None => {
                println!("Error : missing template ref. See tfstarter generate -h");
            }
        },
        None => {
            println!("Please provide argument to use tfstarter. See tfstarter -h for help")
        }
    }
}
