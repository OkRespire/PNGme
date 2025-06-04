use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use std::fs;
use std::path::PathBuf;

use crate::Result;
use crate::chunk::Chunk;
use crate::{chunk_types::ChunkType, png::Png};
use std::convert::TryFrom;
use std::str::FromStr;

pub struct Commands;

impl Commands {
    pub fn new() -> Self {
        Commands {}
    }
    pub fn encode(&self, args: EncodeArgs) -> Result<()> {
        println!(
            "Encoding message '{}' into {:?}",
            args.message, args.file_path
        );

        let file_path = args.file_path;

        let mut png_file = self.open_as_png(&file_path)?;

        let new_chunk_type = ChunkType::from_str(&args.chunk_type)?;

        let byte_msg = args.message.as_bytes();

        let new_chunk = Chunk::new(new_chunk_type, byte_msg.to_vec());

        png_file.append_chunk(new_chunk);

        let output_path = args.output_file.as_ref().unwrap_or(&file_path);
        fs::write(output_path, png_file.as_bytes())?;

        println!("Message successfully encoded into {:?}", output_path);

        Ok(())
    }

    pub fn decode(&self, args: DecodeArgs) -> Result<()> {
        println!("Decoding message from {:?}", args.file_path);

        let file_path = args.file_path;
        let png_file = self.open_as_png(&file_path)?;

        if png_file.chunk_by_type(&args.chunk_type).is_none() {
            return Err("Chunk type not found".into());
        }

        let chunk = png_file.chunk_by_type(&args.chunk_type).unwrap();

        println!("Message = {:?}", chunk.data_as_string().unwrap());

        Ok(())
    }

    pub fn remove(&self, args: RemoveArgs) -> Result<()> {
        println!(
            "Removing chunk {} from {:?}",
            args.chunk_type, args.file_path
        );

        let file_path = args.file_path;
        let mut png_file = self.open_as_png(&file_path)?;

        if !png_file.chunk_by_type(&args.chunk_type).is_none() {
            return Err("Chunk type not found".into());
        }

        png_file.remove_first_chunk(&args.chunk_type)?;

        Ok(())
    }

    pub fn print(&self, args: PrintArgs) -> Result<()> {
        println!("Printing chunks from {:?}", args.file_path);

        let file_path = args.file_path;
        let png_file = self.open_as_png(&file_path)?;

        println!("{}", png_file);

        Ok(())
    }

    /// Helper function to make sure that the file is opened as a png file
    fn open_as_png(&self, file_path: &PathBuf) -> Result<Png> {
        if fs::exists(&file_path).is_err() {
            return Err("File does not exist".into());
        }

        let raw_file = fs::read(file_path)?;
        let u8_raw_file: &[u8] = &raw_file;

        Ok(Png::try_from(u8_raw_file)?)
    }
}
