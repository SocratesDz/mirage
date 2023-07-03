use clap::Parser;
use commands::{Cli, Commands};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode(args) => {
            println!("Used encode command with args: {:?}", args);
        }
        Commands::Decode(args) => {
            println!("Used decode command with args: {:?}", args);
        }
        Commands::Remove(args) => {
            println!("Used remove command with args: {:?}", args);
        }
        Commands::Print(args) => {
            println!("Used print command with args: {:?}", args);
        }
    }

    Ok(())
}
