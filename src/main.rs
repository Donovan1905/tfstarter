use clap::{Parser, Subcommand, Args};

mod init;

#[derive(Parser)]
#[command(author = "donovanhoang.pro@gmail.com", version, about, long_about = None)]
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
    #[arg(short, long, help = "Name of the template you want to use (based on folder name of the template)")]
    template: Option<String> 
}

fn main() {
    let cli = Cli::parse();

    let templates = init::init_app().expect("Failed to load templates");

    match &cli.cmd {
        Some(Commands::Generate(template)) => {
            match template.template {
                Some(ref _template) => {
                    
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
