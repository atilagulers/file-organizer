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
    let entries: ReadDir = fs::read_dir("test_dir")?;
    let mut files: Vec<DirEntry> = Vec::new();

    // extract files
    for entry in entries {
        let entry = entry?;

        if entry.path().is_file() == true {
            println!("{:#?}", entry);
            files.push(entry);
        }
    }

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
