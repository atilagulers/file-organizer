use std::error::Error;
use std::fs::{self, DirEntry, ReadDir};
use std::io;
use std::process;

fn main() {
    // get a folder

    let mut files: Vec<DirEntry> = Vec::new();

    //for entry in entries {
    //    let entry = entry.unwrap();
    //    let metadata = entry.metadata().unwrap();
    //    let is_file = metadata.is_file();
    //    println!("{:#?} {}", entry, is_file);
    //    //files.push(entry);
    //}

    //println!("{:#?}", files[1]);
    // list filenames inside of the folder

    if let Err(e) = run() {
        println!("Application error: {}", e);
        process::exit(1)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(path)?;

    Ok(())
}

//struct Directory {
//    name: String,
//}

//impl Directory {
//    fn new(name: &str) -> Self {
//        Folder {
//            name: name.to_string(),
//        }
//    }
//}
