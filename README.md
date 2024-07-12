# ftree 

`ftree` is a lightweight and convenient utility for displaying a file tree in the terminal. 


## Installation 

In order to install `ftree` you must have Rust and Cargo installed first. If that is already the case, then follow these steps:

1. Clone the repo by running the following 
```bash 
git clone https://github.com/wrp801/ftree.git
```

2. Build the package by navigating to the directory and running `cargo build`

3. It is recommended to install this as a global package so it is accessible outside of the directory. To do so run the following when inside the root of 
the `ftree` directory:
```bash 
cargo install --path .
```

## Usage 

`ftree` takes a single argument of `filepath` which is the path to get the file tree for.
If no filepath is provided, it will use the current working directory as the default. 

### Options 

- To include all files and directories, pass the `-a` or `--all` flag. 

- To include the file size within each entry, pass the `-s` or `--size` flag. This will display the size of each file within each directory

- To include a high level summary, pass the `--summary` flag. This will display the total number of files, directories, and the total size

- To only display the directories within the tree, pass the `-d` or `--dirs` flag


### Patterns 
You can pass a pattern to `ftree` by using the `-p` or `--pattern` flag. This will filter the results of the file tree and only display matches of the pattern. 
If the `-e` or `--exclude` flag is passed, this will ignore any files/directories matching the pattern and thus they will not be displayed in the tree.

