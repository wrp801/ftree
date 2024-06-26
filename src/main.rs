use colored::Colorize;
use clap::Parser;
use std::path::Path;
mod tree;
mod cli;
mod meta; 



fn main() {

    let cli = cli::Args::parse();
    let file_path = cli.file_path.unwrap_or(".".to_string());
    let size = cli.size;

    let mut total_size = u64::default();
    let mut num_files = u64::default();

    println!("{}", file_path.cyan()); // this prints out the starting directory or current working
                               // directory if no argument is passed
    let _ = tree::visit_dirs(&Path::new(&file_path), "", &size, &mut total_size, &mut num_files);
    let total_size_str = tree::get_entry_size(&total_size);
    if size {
        println!("{} Files for a total size of {}", num_files, total_size_str.bright_blue());
    }
    // println!("{} Files for a total size of {}", num_files, total_size_str.bright_blue())

}
