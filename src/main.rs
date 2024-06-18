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
    let directory = Directory::new("test_dir")?;

    println!("{:#?}", directory);
    Ok(())
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
}

impl Directory {
    fn new(name: &str) -> Result<Self, io::Error> {
        let mut directory = Directory {
            name: name.to_string(),
            files: Vec::new(),
        };
        directory.load_files()?;
        Ok(directory)
    }

    fn load_files(&mut self) -> Result<(), io::Error> {
        let entries: ReadDir = fs::read_dir(&self.name)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let extension = path
                    .extension()
                    .and_then(OsStr::to_str)
                    .unwrap_or("")
                    .to_string();
                let name = path
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap_or("")
                    .to_string();
                let file = File::new(&name, &path.to_string_lossy(), &extension);
                self.files.push(file);
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct File {
    name: String,
    path: String,
    extension: String,
}

impl File {
    fn new(name: &str, path: &str, extension: &str) -> Self {
        File {
            name: name.to_string(),
            path: path.to_string(),
            extension: extension.to_string(),
        }
    }
}
