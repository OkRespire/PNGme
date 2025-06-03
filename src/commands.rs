use anyhow::{Error, bail};
use std::fs;

use crate::chunk::Chunk;
use crate::chunk_types::ChunkType;
use crate::png::Png;
use crate::{Error, Result};
use std::convert::TryFrom;
use std::str::FromStr;

pub struct Commands;

impl Commands {
    pub fn encode(&self, args: EncodeArgs) -> Result<(), Error> {
        println!(
            "Encoding message '{}' into {:?}",
            args.message, args.file_path
        );

        let png_file = self.open_as_png(args.file_path)?;

        let new_chunk_type = ChunkType::from_str(args.chunktype)?;

        let byte_msg = args.message.as_bytes();

        let new_chunk = Chunk::new(new_chunk_type, byte_msg);

        png_file.append_chunk(new_chunk);

        let output_path = args.output_file.as_ref().unwrap_or(&args.file_path);
        fs::write(output_path, png_file.as_bytes())?;

        println!("Message successfully encoded into {:?}", output_path);

        Ok(())
    }

    pub fn decode(&self, args: DecodeArgs) -> Result<(), Error> {
        println!("Decoding message from {:?}", args.file_path);

        let png_file = self.open_as_png(args.file_path)?;

        if !png_file.chunk_by_type(&args.message) {
            bail!(Error("Chunk Type does not exist".into()));
        }

        let chunk = png_file.chunk_by_type(&args.message)?;

        println!("Message = {}", chunk.data_as_string());

        Ok(())
    }

    pub fn remove(&self, args: RemoveArgs) -> Result<(), Error> {
        println!(
            "Removing chunk {} from {:?}",
            args.chunk_type, args.file_path
        );

        let png_file = self.open_as_png(args.file_path)?;

        if !png_file.chunk_by_type(&args.message) {
            bail!(Error("Chunk Type does not exist".into()));
        }

        png_file.remove_first_chunk(&args.chunk_type);

        Ok(())
    }

    pub fn print(&self, args: PrintArgs) -> Result<(), Error> {
        // Your print logic here
        println!("Printing chunks from {:?}", args.file_path);
        Ok(())
    }

    /// Helper function to make sure that the file is opened as a png file
    fn open_as_png(&self, file_path: PathBuf) -> Result<Png, Error> {
        if !fs::exists(args.file_path) {
            bail!(Error("File does not exist".into()));
        }

        let raw_file = fs::read(file_path)?;

        Png::try_from(raw_file)
    }
}
