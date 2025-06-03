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

    match args.command {
        PngMeArgs::Encode(args) => Commands::encode(args),
    }
}
