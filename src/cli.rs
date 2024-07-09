use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "ftree",
    version = "0.1.0",
    author = "Wes",
    about = "Prints the directory tree of the given path"
)]
pub struct Args {
    #[clap(name = "FILEPATH", help = "The path to the file to print the tree for")]
    pub file_path: Option<String>,

    // flags
    /// Show all files and directories, including hidden ones. Defaults to false
    #[clap(short = 'a', long = "all", action)]
    pub all: bool,


    /// show the file size for each entry. Defaults to false
    #[clap(short = 's', long = "size", action)]
    pub size: bool,

    /// show a summary of the total number of files, directories, and the total size of all files
    #[clap(long = "summary", help = "Show a summary of the total number of files, directories, and the total size of all files", action)]
    pub summary: bool,

    /// show only the diretories in the tree
    #[clap(short = 'd', long = "dirs", help = "Show only the directories in the tree", action)]
    pub dirs: bool,

    /// The optional pattern of files to include/exclude in the tree
    #[clap(short = 'p',long = "pattern")]
    pub pattern: Option<String>,

    /// the maximum depth of the tree to display
    #[clap(short = 'n', help = "The maximum depth of the tree to display")]
    pub depth: Option<usize>,

    /// A pattern to exclude from the tree
    #[clap(short = 'e',long = "exclude", action)]
    pub exclude: bool


}
