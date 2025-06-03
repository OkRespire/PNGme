use clap::{Args as ClapArgs, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pngme", version, about = "Hides messages in PNG files")]
pub struct Args {
    #[command(subcommand)]
    commands: PngMeArgs,
}

#[derive(Subcommand, Debug)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(ClapArgs, Debug)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(ClapArgs, Debug)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(ClapArgs, Debug)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(ClapArgs, Debug)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}

