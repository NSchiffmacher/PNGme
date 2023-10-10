mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::args::{Args, Commands};
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

use std::fs;
use std::str::FromStr;

use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn encode(filepath: String, chunk_type: String, message: String) -> Result<()> {
    let input_bytes = fs::read(&filepath)?;
    let output = filepath; // For now output is also input

    let mut png = Png::try_from(input_bytes.as_slice())?;
    let chunk = Chunk::new(ChunkType::from_str(&chunk_type[..])?, message.as_bytes().to_vec());
    png.append_chunk(chunk);

    fs::write(output, png.as_bytes())?;
    Ok(())
} 

fn decode(filepath: String, chunk_type: String) -> Result<()> {
    let input_bytes = fs::read(&filepath)?;

    let png = Png::try_from(input_bytes.as_slice())?;
    let chunk = png.chunk_by_type(ChunkType::from_str(&chunk_type[..])?);

    match chunk {
        Some(chunk) => println!("Found hidden message: \"{}\" in chunk \"{}\"", chunk.data_as_string()?, chunk_type.to_string()),
        None => println!("No chunk found with type \"{}\"", chunk_type.to_string())
    }

    Ok(())
}

fn remove(filepath: String, chunk_type: String) -> Result<()> {
    let input_bytes = fs::read(&filepath)?;

    let mut png = Png::try_from(input_bytes.as_slice())?;
    let chunk = png.remove_chunk(ChunkType::from_str(&chunk_type[..])?);

    match chunk {
        Ok(chunk) => {
            println!("Removed hidden message: \"{}\" in chunk \"{}\"", chunk.data_as_string()?, chunk_type.to_string());
            fs::write(filepath, png.as_bytes())?   
        },
        Err(e) => println!("No chunk found with type \"{}\" (got error {})", chunk_type.to_string(), e)
    }

    Ok(())
}

fn print(filepath: String) -> Result<()> {
    let input_bytes = fs::read(&filepath)?;

    let png = Png::try_from(input_bytes.as_slice())?;
    println!("{}", png);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    match args.command {
        Commands::Encode { filepath, chunk_type, message } => encode(filepath, chunk_type, message)?,
        Commands::Decode { filepath, chunk_type } => decode(filepath, chunk_type)?,
        Commands::Remove { filepath, chunk_type } => remove(filepath, chunk_type)?,
        Commands::Print  { filepath } => print(filepath)?,
    };


    Ok(())
}
