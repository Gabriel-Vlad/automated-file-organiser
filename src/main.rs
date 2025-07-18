use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs::{self, DirBuilder, DirEntry, File, OpenOptions, ReadDir};
use std::io::Write;
use std::path::Path;

use crate::constants::{FILES_LOG, MESSY_DIR};

mod constants {
    pub static MESSY_DIR: &str = "messy_folder";
    pub static FILES_LOG: &str = "files.log";
}

fn remove_with_exception<F: AsRef<Path>>(
    entry: DirEntry,
    main_folder: F,
) -> Result<(), Box<dyn Error>> {
    let entry_path = entry.path();
    let entry_path = entry_path
        .file_name()
        .ok_or("Could not retrieve file name")?;

    let main_folder = main_folder.as_ref().as_os_str();
    let archives = OsStr::new("archives");
    let documents = OsStr::new("documents");
    let images = OsStr::new("images");
    let others = OsStr::new("others");

    let special_folders = [main_folder, archives, documents, images, others];

    let is_valid = !special_folders.contains(&entry_path);

    if is_valid {
        fs::remove_dir_all(entry.path())?;
    }

    Ok(())
}

fn is_directory_valid(entry: &DirEntry) -> Result<bool, Box<dyn Error>> {
    let entry_path = entry.path();
    let entry_path = entry_path
        .file_name()
        .ok_or("Could not retrieve file name")?;

    let archives = OsStr::new("archives");
    let documents = OsStr::new("documents");
    let images = OsStr::new("images");
    let others = OsStr::new("others");

    let special_folders = [archives, documents, images, others];

    let is_valid = !special_folders.contains(&entry_path);

    Ok(is_valid)
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

fn parse_entries<F: AsRef<Path>>(
    root: ReadDir,
    main_folder: F,
    files_log: &mut File,
) -> Result<(), Box<dyn Error>> {
    for entry in root {
        let entry = entry?;
        let file_metadata = entry.metadata()?;

        if file_metadata.is_dir() {
            let is_dir_valid = is_directory_valid(&entry)?;

            if is_dir_valid {
                let new_root = fs::read_dir(entry.path())?;

                parse_entries(new_root, main_folder.as_ref(), files_log)?;

                remove_with_exception(entry, main_folder.as_ref())?;
            }
        } else {
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

fn main() -> Result<(), Box<dyn Error>> {
    let root_iterator: ReadDir = fs::read_dir(MESSY_DIR)?;
    let main_folder = Path::new(MESSY_DIR);
    let files_log = Path::new(FILES_LOG);

    let mut files_log = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(files_log)?;

    parse_entries(root_iterator, main_folder, &mut files_log)?;

    println!("Files successfully organized!");
    Ok(())
}
