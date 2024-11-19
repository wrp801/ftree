use colored::Colorize;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use regex::Regex;

/// Recursively visit all directories and files in a directory and print them in a tree format
///
/// * `dir`: The directory to visit
/// * `prefix`: The prefix to use for the tree (either a pipe or a "tab space")
/// * `size`: The flag to print the size of the files
/// * `all`: The flag to print all files including hidden files
/// * `total_size`: The total size of all files in the directory
/// * `num_files`: the total number of files in the directory
/// * `pattern`: The pattern to match files
/// * `exclude`: The flag to exclude files that match the pattern
/// * `dirs`: The flag to only print directories
pub fn visit_dirs(
    dir: &Path,
    prefix: &str,
    size: &bool,
    all: &bool,
    total_size: &mut u64,
    num_files: &mut u64,
    pattern: &str,
    exclude: &bool,
    dirs: &bool,
    include: &String
) -> io::Result<()> {
    if dir.is_dir() {
        // collect all entries in the directory and sort them alphabetically
        let mut entries = fs::read_dir(dir)?.collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort_by_key(|dir| dir.path());
        if !*all {
            entries = entries
                .into_iter()
                .filter(|entry| !is_hidden(entry))
                .collect();
        }

        if !pattern.is_empty() {
            if *exclude {
                entries = entries
                    .into_iter()
                    .filter(|entry| !is_match(pattern, entry))
                    .collect();
            } else {
                entries = entries
                    .into_iter()
                    .filter(|entry| is_match(pattern, entry))
                    .collect();
            }
        }
        if *dirs {
            entries = entries
                .into_iter()
                .filter(|entry| entry.path().is_dir())
                .collect();
        }
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
                match entry_size {
                    Some(size) => println!(
                        "{}{}{} ({})",
                        prefix,
                        connector,
                        path.file_name().unwrap().to_str().unwrap(),
                        &size.bright_blue()
                    ),
                    None => println!(
                        "{}{}{}",
                        prefix,
                        connector,
                        path.file_name().unwrap().to_str().unwrap()
                    ),
                }
            } else {
                println!(
                    "{}{}{}",
                    prefix,
                    connector,
                    path.file_name().unwrap().to_str().unwrap()
                );
            }
            if path.is_dir() {
                let new_prefix = format!("{}{}", prefix, new_prefix);
                visit_dirs(&path, &new_prefix, &size, &all, total_size, num_files, &pattern, &exclude, &dirs, &include)?;
            }
        }
    }
    Ok(())
}

/// This function returns the size of a file in a human readable format
///
/// * `size`: The size of a file in bytes
pub fn get_entry_size(size: &u64) -> String {
    match size {
        0..=1024 => format!("{} Bytes", size),
        1025..=1048576 => format!("{:.2} KB", *size as f64 / 1024.0),
        1048577..=1073741824 => format!("{:.2} MB", *size as f64 / 1048576.0),
        _ => format!("{:.2} GB", *size as f64 / 1073741824.0),
    }
}

///  This function returns the size of a file in a human readable format. Will return None if the
///  entry is a directory
///
/// * `entry`: The entry to get the size of
fn get_file_size(entry: &DirEntry) -> Option<String> {
    if entry.metadata().unwrap().is_file() {
        let size = entry.metadata().unwrap().len();
        match size {
            0..=1024 => Some(format!(
                "{} {}",
                size.to_string().bright_blue(),
                "Bytes".bright_blue()
            )),
            1025..=1048576 => {
                let fsize = size as f64 / 1024.0;
                let fsize_str = format!("{:.2}", fsize);
                Some(format!(
                    "{} {}",
                    fsize_str.bright_green(),
                    "KB".bright_green()
                ))
            }
            1048577..=1073741824 => {
                let fsize = size as f64 / 1048576.0;
                let fsize_str = format!("{:.2}", fsize);
                Some(format!(
                    "{} {}",
                    fsize_str.bright_yellow(),
                    "MB".bright_yellow()
                ))
            }
            _ => {
                let fsize = size as f64 / 1073741824.0;
                let fsize_str = format!("{:.2}", fsize);
                Some(format!(
                    "{} {}",
                    fsize_str.bright_purple(),
                    "GB".bright_purple()
                ))
            }
        }
    } else {
        None
    }
}

/// This function updates the total size of all files in the directory
/// The total size will be printed at the end of the output
fn update_total_size(entry: &DirEntry, total_size: &mut u64) {
    let size = entry.metadata().unwrap().len();
    *total_size += size;
}

/// Helper function to check if a file is a hidden file
///
/// * `entry`: The entry to check
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().unwrap().starts_with(".")
}

pub fn scan_dirs(
    dir: &Path,
    size: &bool,
    all: &bool,
    total_size: &mut u64,
    num_files: &mut u64,
    total_dirs: &mut u64,
) -> io::Result<()> {
    if dir.is_dir() {
        // collect all entries in the directory and sort them alphabetically
        let mut entries = fs::read_dir(dir)?.collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort_by_key(|dir| dir.path());
        if !*all {
            entries = entries
                .into_iter()
                .filter(|entry| !is_hidden(entry))
                .collect();
        }
        let n_contents = entries.iter().count();
        // capture the total number of files in the directory
        *num_files += n_contents as u64;
        *total_dirs += 1 as u64;

        for entry in entries {
            let path = entry.path();
            update_total_size(&entry, total_size);
            // if *size {
            //     update_total_size(&entry, total_size);
            // }
            if path.is_dir() {
                scan_dirs(&path, &size, &all, total_size, num_files, total_dirs)?;
            }
        }
    }
    Ok(())
}

fn is_match(pattern: &str, entry: &DirEntry) -> bool {
    let re = Regex::new(pattern).unwrap();
    re.is_match(entry.file_name().to_str().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_entry_size() {
        let size = 1024;
        assert_eq!(get_entry_size(&size), "1024 Bytes");
        let size = 1048576;
        assert_eq!(get_entry_size(&size), "1024.00 KB");
        let size = 1073741824;
        assert_eq!(get_entry_size(&size), "1024.00 MB");
        let size = 1099511627776;
        assert_eq!(get_entry_size(&size), "1024.00 GB");
    }

    #[test]
    fn test_is_hidden() {
        let entry = fs::read_dir(".").unwrap().next().unwrap().unwrap();
        assert_eq!(is_hidden(&entry), false);
    }

    #[test] 
    fn test_regex_match() {
        // let entry = fs::read_dir("tests/testdir/").unwrap().next().unwrap().unwrap();
        let entries = fs::read_dir("tests/testdir").unwrap().collect::<Result<Vec<_>, io::Error>>().unwrap();
        let pattern = "csv";
        let filtered = entries
            .into_iter()
            .filter(|entry| is_match(pattern, entry))
            .collect::<Vec<_>>();
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_regex_exclude() {
        let entries = fs::read_dir("tests/testdir").unwrap().collect::<Result<Vec<_>, io::Error>>().unwrap();
        let pattern = "example_1.csv";
        let filtered = entries
            .into_iter()
            .filter(|entry| !is_match(pattern, entry))
            .collect::<Vec<_>>();
        println!("{:?}", filtered);
        assert_eq!(filtered.len(), 2);
    }
}



