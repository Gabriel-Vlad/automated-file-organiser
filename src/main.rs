use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, DirBuilder, DirEntry, OpenOptions, ReadDir};
use std::io::Write;
use std::path::Path;

use crate::constants::{FILES_LOG, MESSY_DIR};

mod constants {
    pub static MESSY_DIR: &'static str = "messy_folder";
    pub static FILES_LOG: &'static str = "files.log";
}

fn parse_entries<F: AsRef<Path>>(
    root: ReadDir,
    main_folder: F,
    files_log: F,
) -> Result<(), Box<dyn Error>> {
    for entry in root {
        let entry = entry?;
        let file_metadata = entry.metadata()?;

        if file_metadata.is_dir() {
            let new_root = fs::read_dir(entry.path())?;
            parse_entries(new_root, main_folder.as_ref(), files_log.as_ref())?;

            if entry.file_name() != main_folder.as_ref() {
                fs::remove_dir(entry.path())?;
            }
        } else {
            let mut files_log = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(files_log.as_ref())?;

            // let mut log_string = OsString::new();

            // log_string.push(entry.path().as_os_str());

            if let Some(ext) = entry.path().extension() {
                match ext.to_str().ok_or("Something went wrong")? {
                    "png" | "jpeg" | "jpg" => {
                        // log_string.push("- image");
                        change_file_path(entry, main_folder.as_ref(), "images")?;
                    }

                    "pdf" | "txt" => {
                        // log_string.push("- document");
                        change_file_path(entry, main_folder.as_ref(), "documents")?;
                    }

                    "zip" => {
                        // log_string.push("- archive");
                        change_file_path(entry, main_folder.as_ref(), "archives")?;
                    }

                    _ => {
                        // log_string.push("- others");
                        change_file_path(entry, main_folder.as_ref(), "others")?;
                    }
                }
            }

            // log_string.push("\n");
            // files_log.write_all(log_string.as_encoded_bytes())?;
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
    parse_entries(root, main_folder, files_log)?;
    Ok(())
}
