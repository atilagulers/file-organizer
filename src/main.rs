use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, DirEntry, ReadDir};
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use walkdir::WalkDir;

fn main() {
    // get a folder

    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    const DIRECTORY_PATH: &str = "test_dir";

    let dir_path: PathBuf = PathBuf::from(DIRECTORY_PATH);
    let mut files = walk_dir(&dir_path)?;

    let mut extensions: HashMap<String, u32> = HashMap::new();

    for file in &mut files {
        let counter = extensions.entry(file.extension.clone()).or_insert(0);

        *counter += 1;
    }

    let parent_dir = dir_path.parent().ok_or("Parent directory not found")?;

    let formatted_dir = parent_dir.join("formatted_files");

    //match fs::create_dir(formatted_dir) {
    //    Ok(x) => println!("{:#?}", x),
    //    Err(e) => println!("{:#?}", e),
    //}

    fs::create_dir(&formatted_dir).or_else(|e| {
        if e.kind() == io::ErrorKind::AlreadyExists {
            Ok(())
        } else {
            Err(e)
        }
    })?;

    for ext in extensions.keys() {
        let ext_dir = formatted_dir.join(ext);
        fs::create_dir(&ext_dir).or_else(|e| {
            if e.kind() == io::ErrorKind::AlreadyExists {
                Ok(())
            } else {
                Err(e)
            }
        })?;
    }

    //println!("{:#?}", new_folder);

    Ok(())
}

fn walk_dir(path: &PathBuf) -> Result<Vec<File>, Box<dyn Error>> {
    let dir: ReadDir = fs::read_dir(path)?;
    let mut files: Vec<File> = vec![];

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_files = walk_dir(&path)?;
            files.extend(sub_files);
        } else if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                let extension = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("no_extension");
                let file = File::new(path.to_str().unwrap(), file_name, extension);
                files.push(file);
                //println!("{:#?} is pushed to files", file);
            }
        }
    }

    Ok(files)
}

#[derive(Debug)]
struct File {
    path: String,
    file_name: String,
    extension: String,
}

impl File {
    fn new(path: &str, file_name: &str, extension: &str) -> Self {
        File {
            path: path.to_string(),
            file_name: file_name.to_string(),
            extension: extension.to_string(),
        }
    }
}

//for entry in WalkDir::new(DIRECTORY_PATH)
//    .into_iter()
//    .filter_map(|e| e.ok())
//{
//    let path = entry.path();
//    if path.is_file() {
//        println!("{}", path.display());
//    }
//}

// use std::collections::HashMap;
// use std::error::Error;
// use std::ffi::OsStr;
// use std::fs::{self, DirEntry, ReadDir};
// use std::io;
// use std::path::Path;
// use std::process;

// fn main() {
//     // get a folder

//     if let Err(e) = run() {
//         eprintln!("Application error: {}", e);
//         process::exit(1)
//     }
// }

// fn run() -> Result<(), Box<dyn Error>> {
//     const DIRECTORY_PATH: &str = "testo";

//     let directory = fs::read_dir(DIRECTORY_PATH)?;
//     let mut extensions = HashMap::new();

//     for entry in directory {
//         let entry = entry?;
//         let path = entry.path();
//         let file_name = entry.file_name();

//         let extension = path
//             .extension()
//             .and_then(|ext| ext.to_str())
//             .map(|s| s.to_string())
//             .unwrap_or_else(|| "no_extension".to_string());

//         let counter = extensions.entry(extension.clone()).or_insert(0);
//         *counter += 1;

//         println!("{:#?}", path);
//     }

//     for extension in extensions.keys() {
//         let dir_path: String = format!("{}/{}", DIRECTORY_PATH, extension);
//         fs::create_dir_all(&dir_path)?;
//     }

//     // println!("{:#?}:", directory);
//     Ok(())
// }

// #[derive(Debug)]
// struct Directory {
//     name: String,
//     files: Vec<File>,
// }

// impl Directory {
//     fn new(name: &str) -> Result<Self, io::Error> {
//         let mut directory = Directory {
//             name: name.to_string(),
//             files: Vec::new(),
//         };
//         // directory.load_files()?;
//         Ok(directory)
//     }

//     fn load_files(&mut self) -> Result<(), io::Error> {
//         let entries: ReadDir = fs::read_dir(&self.name)?;

//         for entry in entries {
//             let entry = entry?;
//             let path = entry.path();
//             if path.is_file() {
//                 let extension = path
//                     .extension()
//                     .and_then(OsStr::to_str)
//                     .unwrap_or("")
//                     .to_string();
//                 let name = path
//                     .file_name()
//                     .and_then(OsStr::to_str)
//                     .unwrap_or("")
//                     .to_string();
//                 let file = File::new(&name, &path.to_string_lossy(), &extension);
//                 self.files.push(file);
//             }
//         }
//         Ok(())
//     }
// }

// #[derive(Debug)]
// struct File {
//     name: String,
//     path: String,
//     extension: String,
// }

// impl File {
//     fn new(name: &str, path: &str, extension: &str) -> Self {
//         File {
//             name: name.to_string(),
//             path: path.to_string(),
//             extension: extension.to_string(),
//         }
//     }
// }
