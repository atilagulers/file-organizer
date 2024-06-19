use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, ReadDir};
use std::io;
use std::path::PathBuf;
use std::process;

fn main() {
    // get a folder

    eprintln!("Please provide a folder path");

    let target_path = io::stdin()
        .lines()
        .next()
        .expect("No input found")
        .expect("Failed to read line");

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        if let Err(e) = run(&target_path) {
            eprintln!("Application error: {}", e);
            process::exit(1)
        }

        if i % 5 == 0 {
            pb.println(format!("[+] finished #{}", i));
        }
        pb.inc(5);
    }
    pb.finish_with_message("done");
}

fn run(target_path: &str) -> Result<(), Box<dyn Error>> {
    let dir_path: PathBuf = PathBuf::from(target_path);
    let mut files = walk_dir(&dir_path)?;

    let mut extensions: HashMap<String, u32> = HashMap::new();

    for file in &mut files {
        let counter = extensions.entry(file.extension.clone()).or_insert(0);

        *counter += 1;
    }

    let parent_dir = dir_path.parent().ok_or("Parent directory not found")?;

    let formatted_dir = parent_dir.join("formatted_files");

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

    for file in files {
        let ext_dir = formatted_dir.join(&file.extension);
        let new_file_path = ext_dir.join(&file.file_name);
        fs::copy(&file.path, &new_file_path)?;
    }

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
