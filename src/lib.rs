
pub mod args;
pub mod chunk;
pub mod chunk_types;
pub mod commands;
pub mod png;

pub use commands::Commands;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
pub fn decode_with_args(args: crate::args::DecodeArgs) -> Result<String> {
    let runner = Commands::new();
    runner.decode(args)
}

pub fn encode_with_args(args: crate::args::EncodeArgs) -> Result<String> {
    let runner = Commands::new();
    runner.encode(args)
}

pub fn remove_with_args(args: crate::args::RemoveArgs) -> Result<String> {
    let runner = Commands::new();
    runner.remove(args)
}

pub fn print_with_args(args: crate::args::PrintArgs) -> Result<String> {
    let runner = Commands::new();
    runner.print(args)
}
