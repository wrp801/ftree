use clap::{Parser, ArgGroup, Arg};

#[derive(Debug,Parser)]
#[clap(name = "ftree", version = "0.1.0", author = "Wes", about = "Prints the directory tree of the given path")]
pub struct Args {
    #[clap(name = "FILEPATH", help = "The path to the file to print the tree for")]
    pub file_path: Option<String>,

    // flags 
    /// Show all files and directories, including hidden ones. Defaults to false
    #[clap(short = 'a', long = "all", action)]
    pub hidden:bool,

    /// Colorize the output. Defaults to false
    #[clap(short = 'c', long = "color", help = "Colorize the output", action)]
    pub color:bool,

    
    /// show the file size for each entry. Defaults to false
    #[clap(short = 's', long = "size", action)]
    // NOTE: this should also show the total number of files found and total size of all files at
    // the end of the output
    pub size:bool
}

