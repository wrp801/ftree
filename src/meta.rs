use std::fs::DirEntry;

struct FTreeMetadata {
    total_files: u64,
    total_dirs: u64,
    total_size: u64,
}

impl FTreeMetadata {
    fn new() -> Self {
        FTreeMetadata {
            total_files: u64::default(),
            total_dirs: u64::default(),
            total_size: u64::default(),
        }
    }

    fn update(&mut self, entry: &DirEntry) {
        if entry.path().is_dir() {
            self.total_dirs += 1;
        } else {
            self.total_files += 1;
            update_total_size(entry, &mut self.total_size);
        }
    }

    fn print(&self) {
        println!(
            "\n{} directories, {} files, total size: {}",
            self.total_dirs, self.total_files, get_entry_size(&self.total_size)
        );
    }
}

/// This function updates the total size of all files in the directory
/// The total size will be printed at the end of the output
fn update_total_size(entry: &DirEntry, total_size: &mut u64) {
    let size = entry.metadata().unwrap().len();
    *total_size += size;
}

fn get_entry_size(size: &u64) -> String {
    match size {
        0..=1024 => format!("{} Bytes", size),
        1024..=1048576 => format!("{:.2} KB", *size as f64 / 1024.0),
        1048576..=1073741824 => format!("{:.2} MB", *size as f64 / 1048576.0),
        _ => format!("{:.2} GB", *size as f64 / 1073741824.0),
    }
}

