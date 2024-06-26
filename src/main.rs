use clap::Parser;
use colored::Colorize;
use std::path::Path;
mod cli;
mod meta;
mod tree;

fn main() {
    let cli = cli::Args::parse();
    let file_path = cli.file_path.unwrap_or(".".to_string());
    let size = cli.size;
    let all = cli.all;
    let summary = cli.summary;

    let mut total_size = u64::default();
    let mut num_files = u64::default();
    let mut total_dirs = u64::default();


    println!("{}", file_path.cyan()); // this prints out the starting directory or current working
                                      // directory if no argument is passed
    if summary {
        let _ = tree::scan_dirs(&Path::new(&file_path), &size, &all, &mut total_size, &mut num_files, &mut total_dirs);
        println!("Total files: {}", num_files);
        println!("Total directories: {}", total_dirs);
        println!("Total size of contents: {}", total_size);

        
    } else {
        let _ = tree::visit_dirs(
            &Path::new(&file_path),
            "",
            &size,
            &all,
            &mut total_size,
            &mut num_files,
        );
        let total_size_str = tree::get_entry_size(&total_size);
        if size {
            println!(""); // blank line for readability
            println!(
                "{} Files for a total size of {}",
                num_files,
                total_size_str.bright_cyan()
            );
        }
    }
}
