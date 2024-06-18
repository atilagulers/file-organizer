use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, DirEntry, ReadDir};
use std::io;
use std::path::Path;
use std::process;

fn main() {
    // get a folder

    if let Err(e) = run() {
        println!("Application error: {}", e);
        process::exit(1)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut directory = Directory::new("test_dir");

    directory.files = get_files("test_dir")?;

    println!("{:#?}", directory.files);
    Ok(())
}

fn get_files(dir: &str) -> Result<Vec<DirEntry>, io::Error> {
    let entries: ReadDir = fs::read_dir(dir)?;
    let mut files: Vec<DirEntry> = Vec::new();

    let path = Path::new(dir);

    // extract files
    for entry in entries {
        if let Ok(entry) = entry {
            println!("{:?}", entry.path());
        }
    }

    Ok(files)
}

struct Directory {
    name: String,
    files: Vec<DirEntry>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            files: Vec::new(),
        }
    }
}
