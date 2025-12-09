use clap::{Parser, Subcommand};
use std::process::Command;


use morse_rs::{decode, encode};
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode { value: String },
    Decode { value: String },
    Learn,
}

fn main() {
    Command::new("clear")
        .status()
        .expect("error on clear terminal");

    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { value } => {
            println!(
                "Text: {} to morse is:  ..- --. ---. --.-",
                encode::encode(value)
            )
        }
        Commands::Decode { value } => {
            println!("{}", decode::decode(value))
        }
        Commands::Learn => {
            println!("Learning progress")
        }
    }
}
