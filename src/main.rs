use std::fs::{self, DirEntry};
use std::process;

fn main() {
    // get a folder
    let entries = fs::read_dir("test_dir").unwrap_or_else(|err| {
        println!("Failed to read directory, Error: {}", err);
        process::exit(1);
    });

    let mut files: Vec<DirEntry> = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        println!("{:?}", entry);
        files.push(entry);
    }

    println!("{:#?}", files[1].file_type());
    // list filenames inside of the folder
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
