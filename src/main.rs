use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs::{self, DirBuilder, DirEntry, OpenOptions, ReadDir};
use std::io::Write;
use std::path::Path;

use crate::constants::{FILES_LOG, MESSY_DIR};

mod constants {
    pub static MESSY_DIR: &'static str = "messy_folder";
    pub static FILES_LOG: &'static str = "files.log";
}

fn remove_with_exception<F: AsRef<Path>>(
    entry: DirEntry,
    main_folder: F,
) -> Result<(), Box<dyn Error>> {
    println!("{:?}", entry.path());

    let entry_path = entry.path();
    let entry_path = entry_path.as_os_str();
    let main_folder = main_folder.as_ref().as_os_str();
    let archives = OsString::from("archives");
    let documents = OsString::from("documents");
    let images = OsString::from("images");
    let others = OsString::from("others");

    let is_valid = !(entry_path == main_folder
        || entry_path == archives
        || entry_path == documents
        || entry_path == images
        || entry_path == others);

    if !is_valid {
        fs::remove_dir(entry.path())?;
    }

    Ok(())
}

fn ignore_directories(entry: &DirEntry) -> Result<bool, Box<dyn Error>> {
    let file_metadata = entry.metadata()?;

    let entry_path = entry.path();
    let entry_path = entry_path.as_os_str();
    let archives = OsStr::new("archives");
    let documents = OsStr::new("documents");
    let images = OsStr::new("images");
    let others = OsStr::new("others");

    let is_valid = file_metadata.is_dir()
        && !(entry_path == archives
            || entry_path == documents
            || entry_path == images
            || entry_path == others);

    Ok(is_valid)
}

fn parse_entries<F: AsRef<Path>>(
    root: ReadDir,
    main_folder: F,
    files_log: F,
) -> Result<(), Box<dyn Error>> {
    for entry in root {
        let entry = entry?;
        let is_dir_valid = ignore_directories(&entry)?;

        if is_dir_valid {
            let new_root = fs::read_dir(entry.path())?;

            parse_entries(new_root, main_folder.as_ref(), files_log.as_ref())?;

            remove_with_exception(entry, main_folder.as_ref())?;
        } else {
            let mut files_log = OpenOptions::new()
                .write(true)
                .append(true)
                .open(files_log.as_ref())?;

            let mut log_string = OsString::new();

            log_string.push(entry.path().as_os_str());

            if let Some(ext) = entry.path().extension() {
                match ext.to_str().ok_or("Something went wrong")? {
                    "png" | "jpeg" | "jpg" => {
                        log_string.push("- image");
                        change_file_path(entry, main_folder.as_ref(), "images")?;
                    }

                    "pdf" | "txt" => {
                        log_string.push("- document");
                        change_file_path(entry, main_folder.as_ref(), "documents")?;
                    }

                    "zip" => {
                        log_string.push("- archive");
                        change_file_path(entry, main_folder.as_ref(), "archives")?;
                    }

                    _ => {
                        log_string.push("- others");
                        change_file_path(entry, main_folder.as_ref(), "others")?;
                    }
                }
            }

            log_string.push("\n");
            files_log.write_all(log_string.as_encoded_bytes())?;
        }
    }

    Ok(())
}

fn change_file_path<F: AsRef<Path>>(
    entry: DirEntry,
    main_folder: F,
    new_folder_name: &str,
) -> Result<(), Box<dyn Error>> {
    let new_dir = main_folder.as_ref().join(new_folder_name);
    DirBuilder::new().recursive(true).create(&new_dir)?;

    let entry_path = entry.path();

    let file_name = entry_path
        .file_name()
        .ok_or("Could not retrieve file name")?;

    let new_path = new_dir.join(file_name);

    fs::rename(entry.path(), new_path)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let root: ReadDir = fs::read_dir(MESSY_DIR)?;
    let main_folder = Path::new(MESSY_DIR);
    let files_log = Path::new(FILES_LOG);

    {
        OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(files_log)?;

        println!("Success");
    }

    parse_entries(root, main_folder, files_log)?;
    Ok(())
}
