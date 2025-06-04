use clap::Parser;
mod args;
mod chunk;
mod chunk_types;
mod commands;
mod png;
use args::{Args, PngMeArgs};
use commands::Commands;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();
    let command_runner = Commands::new();

    match args.commands {
        PngMeArgs::Encode(args) => command_runner.encode(args)?,
        PngMeArgs::Decode(args) => command_runner.decode(args)?,
        PngMeArgs::Remove(args) => command_runner.remove(args)?,
        PngMeArgs::Print(args) => command_runner.print(args)?,
    }
    Ok(())
}
