use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use dotenv::dotenv;
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
    New(New),
    Update(Update),
}

#[derive(Args)]
struct New {
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
struct Update {}


fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    init::init_app().expect("Failed to load templates");

    let home_dir = env::var("HOME").unwrap();
    let mut src = PathBuf::from(home_dir);
    src.push(".tfstarter/");

    match &cli.cmd {
        Some(Commands::New(template)) => match template.template {
            Some(ref _template) => {
                src.push(_template);
                commands::replace(src).expect("Failed to replace placeholder in template");
                println!(
                    "\nReplaced placeholder in template {}, copied in current directory",
                    _template.bold().green()
                );
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
        Some(Commands::Update(_arg)) => {
            commands::update(src).expect("Failed to update default templates folder");
            println!("{}", "Successfully update templates !".green().bold());
        }
        None => {
            println!("Please provide argument to use tfstarter. See tfstarter -h for help")
        }
    }
}
