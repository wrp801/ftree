use std::fs::{self, DirEntry};
use std::path::Path;
use std::io;
use colored::Colorize;

pub fn visit_dirs(dir: &Path, prefix: &str, size: &bool, total_size: &mut u64, num_files: &mut u64) -> io::Result<()> {
    if dir.is_dir() {
        // collect all entries in the directory and sort them alphabetically
        let mut entries = fs::read_dir(dir)?.collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort_by_key(|dir| dir.path()); 
        let n_contents = entries.iter().count();
        // capture the total number of files in the directory
        *num_files += n_contents as u64;
        
        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_last = i == entries.len() - 1; // Check if it's the last entry
            let new_prefix = if is_last { "    " } else { "│   " };
            let connector = if is_last { "└── " } else { "├── " };
            if *size {
                let entry_size = get_file_size(entry);
                update_total_size(entry, total_size);
                println!("{}{}{} ({})", prefix, connector, path.file_name().unwrap().to_str().unwrap(),&entry_size.bright_blue());
            }
            else {
                println!("{}{}{}", prefix, connector, path.file_name().unwrap().to_str().unwrap());
            }
            if path.is_dir() {
                let new_prefix = format!("{}{}", prefix, new_prefix);
                visit_dirs(&path, &new_prefix, &size, total_size, num_files)?;
            }
        }
    }
    Ok(())
}



pub fn get_entry_size(size: &u64) -> String {
    match size {
        0..=1024 => format!("{} Bytes", size),
        1025..=1048576 => format!("{:.2} KB", *size as f64 / 1024.0),
        1048577..=1073741824 => format!("{:.2} MB", *size as f64 / 1048576.0),
        _ => format!("{:.2} GB", *size as f64 / 1073741824.0),
    }
}

fn get_file_size(entry: &DirEntry) -> String {
    let size = entry.metadata().unwrap().len();
    match size {
        0..=1024 => format!("{} Bytes", size),
        1025..=1048576 => format!("{:.2} KB", size as f64 / 1024.0),
        1048577..=1073741824 => format!("{:.2} MB", size as f64 / 1048576.0),
        _ => format!("{:.2} GB", size as f64 / 1073741824.0),
    }
}


/// This function updates the total size of all files in the directory
/// The total size will be printed at the end of the output
fn update_total_size(entry: &DirEntry, total_size: &mut u64) {
    let size = entry.metadata().unwrap().len();
    *total_size += size;
}



fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().unwrap().starts_with(".")
}

