use PNGme::{
    Commands, Result,
    args::{Args, PngMeArgs},
};
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    let command_runner = Commands::new();

    match args.commands {
        PngMeArgs::Encode(args) => command_runner.encode(args)?,
        PngMeArgs::Decode(args) => command_runner.decode(args)?,
        PngMeArgs::Remove(args) => command_runner.remove(args)?,
        PngMeArgs::Print(args) => command_runner.print(args)?,
    };
    Ok(())
}
