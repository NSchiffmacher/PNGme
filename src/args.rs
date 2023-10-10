use clap::{Parser, Subcommand};

use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encodes a message into a PNG file
    Encode { 
        /// path to the PNG file 
        filepath: String, 

        /// 4-letter chunk type
        chunk_type: String, 

        /// message to add to the png file
        message: String,

        // /// output file
        // out: Option<String>
    },
    
    /// Decodes a message from a given chunk in a PNG file
    Decode { 
        /// path to the PNG file 
        filepath: String, 

        /// 4-letter chunk type
        chunk_type: String,
    },

    /// Removes a chunk from a PNG file 
    Remove { 
        /// path to the PNG file 
        filepath: String, 
        
        /// 4-letter chunk type
        chunk_type: String, 
    },

    /// Prints the content of a given png file
    Print { 
        /// path to the PNG file 
        filepath: String, 
    },
}
/// Simple program to encode/decode hidden messages in PNG files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command to execute 
    #[command(subcommand)]
    pub command: Commands,
}
