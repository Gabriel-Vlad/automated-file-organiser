mod constants;

use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs::{self, DirBuilder, DirEntry, File, ReadDir};
use std::io::Write;
use std::path::Path;

use constants::*;

// -- The main function responsible for organizing the files
// by their extensions into their own named directories --
pub fn parse_entries<F: AsRef<Path>>(
    root: ReadDir,
    directory_to_organize: F,
    files_log: &mut File,
) -> Result<(), Box<dyn Error>> {
    // -- Iterating over the entires and files --
    for entry in root {
        let entry = entry?;
        let file_metadata = entry.metadata()?;

        if file_metadata.is_dir() {
            // -- Verifies if the directory can be
            // traversed and modified --
            let is_dir_valid = is_directory_valid(&entry)?;

            if is_dir_valid {
                let new_root = fs::read_dir(entry.path())?;

                // -- Recursively exploring the current directory --
                parse_entries(new_root, directory_to_organize.as_ref(), files_log)?;

                // -- Removes the old directories after they have been
                // emptied completely unless specified --
                remove_with_exception(entry, directory_to_organize.as_ref())?;
            }
        } else {
            let mut log_string = OsString::new();

            log_string.push(entry.path().as_os_str());

            // -- Organizing the files in each directory the belong to --
            if let Some(ext) = entry.path().extension() {
                match ext.to_str().ok_or("Something went wrong")? {
                    ext if IMAGE_EXTENSIONS.contains(&ext) => {
                        log_string.push(" - IMAGE");
                        change_file_path(entry, directory_to_organize.as_ref(), "images")?;
                    }

                    ext if DOCUMENT_EXTENSIONS.contains(&ext) => {
                        log_string.push(" - DOCUMENT");
                        change_file_path(entry, directory_to_organize.as_ref(), "documents")?;
                    }

                    ext if ARCHIVES_EXTENSIONS.contains(&ext) => {
                        log_string.push(" - ARCHIVE");
                        change_file_path(entry, directory_to_organize.as_ref(), "archives")?;
                    }

                    ext if DATA_INTERCHANGE_FORMAT_EXTENSIONS.contains(&ext) => {
                        log_string.push(" - DATA-INTERCHANGE");
                        change_file_path(
                            entry,
                            directory_to_organize.as_ref(),
                            "data_interchange",
                        )?;
                    }

                    ext if AUDIO_EXTENSIONS.contains(&ext) => {
                        log_string.push(" - AUDIO");
                        change_file_path(entry, directory_to_organize.as_ref(), "audio")?;
                    }

                    ext if VIDEO_EXTENSIONS.contains(&ext) => {
                        log_string.push(" - VIDEO");
                        change_file_path(entry, directory_to_organize.as_ref(), "video")?;
                    }

                    ext if SOURCE_CODE_EXTENSIONS.contains(&ext) => {
                        log_string.push(" - SOURCE-CODE");
                        change_file_path(entry, directory_to_organize.as_ref(), "source_code")?;
                    }

                    ext if PRESENTATION_EXTENSIONS.contains(&ext) => {
                        log_string.push(" - PRESENTATION");
                        change_file_path(entry, directory_to_organize.as_ref(), "presentations")?;
                    }

                    ext if FONT_EXTENSIONS.contains(&ext) => {
                        log_string.push(" - FONT");
                        change_file_path(entry, directory_to_organize.as_ref(), "fonts")?;
                    }

                    _ => {
                        log_string.push(" - others");
                        change_file_path(entry, directory_to_organize.as_ref(), "others")?;
                    }
                }
            }

            log_string.push("\n");
            files_log.write_all(log_string.as_encoded_bytes())?;
        }
    }

    Ok(())
}

fn remove_with_exception<F: AsRef<Path>>(
    entry: DirEntry,
    main_folder: F,
) -> Result<(), Box<dyn Error>> {
    let entry_path = entry.path();
    let entry_path = entry_path
        .file_name()
        .ok_or("Could not retrieve file name")?;

    let mut special_folders = SPECIAL_FOLDERS
        .iter()
        .map(OsStr::new)
        .collect::<Vec<&OsStr>>();
    special_folders.push(main_folder.as_ref().as_os_str());

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

    let special_folders = SPECIAL_FOLDERS
        .iter()
        .map(OsStr::new)
        .collect::<Vec<&OsStr>>();

    let is_valid = !special_folders.contains(&entry_path);

    Ok(is_valid)
}

fn change_file_path<F: AsRef<Path>>(
    entry: DirEntry,
    directory_to_organize: F,
    new_folder_name: &str,
) -> Result<(), Box<dyn Error>> {
    let new_dir = directory_to_organize.as_ref().join(new_folder_name);
    DirBuilder::new().recursive(true).create(&new_dir)?;

    let entry_path = entry.path();

    let file_name = entry_path
        .file_name()
        .ok_or("Could not retrieve file name")?;

    let new_path = new_dir.join(file_name);

    fs::rename(entry.path(), new_path)?;

    Ok(())
}
