use std::{fs, process};

fn main() {
    println!("Hello, world!");

    // get a folder
    let folder = fs::read_dir("test_dir").unwrap_or_else(|err| {
        println!("Failed to read directory, Error: {}", err);
        process::exit(1);
    });

    for entry in folder {
        println!("Entry: {:?}", entry);
    }

    // list filenames inside of the folder
}
