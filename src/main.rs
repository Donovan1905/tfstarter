use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use std::{env, fs, io, path::PathBuf};

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
                let dst = env::current_dir().unwrap();
                utils::copy_dir_all(src, dst).unwrap();
                println!("{}", _template);
            }
            None => {
                println!("Error : missing template ref. See tfstarter generate -h");
            }
        },
        Some(Commands::Get(ttype)) => match ttype.ttype {
            Some(ref _ttype) => {
                let entries = fs::read_dir(src)
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
            }
            None => {
                println!("Error : missing type")
            }
        },
        None => {
            println!("Please provide argument to use tfstarter. See tfstarter -h for help")
        }
    }
}
