use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "The Stateless Key Management System for ezeX platform", long_about = None)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Starts the services")]
    Start,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => {
            println!("App started!");
        }
    }
}
