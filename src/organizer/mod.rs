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
    logger_file: &mut File,
    perform_cleanup: bool,
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
                parse_entries(
                    new_root,
                    directory_to_organize.as_ref(),
                    logger_file,
                    perform_cleanup,
                )?;

                // -- Removes the old directories after they have been
                // emptied completely unless specified --
                if perform_cleanup {
                    perform_cleanup_with_exceptions(entry, directory_to_organize.as_ref())?;
                }
            }
        } else {
            let mut log_string = OsString::new();

            log_string.push(entry.path().as_os_str());

            // -- Organizing the files in each directory the belong to --
            if let Some(ext) = entry.path().extension() {
                match ext.to_str().ok_or("Something went wrong")? {
                    ext if IMAGE_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "images",
                        &mut log_string,
                    )?,

                    ext if DOCUMENT_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "documents",
                        &mut log_string,
                    )?,

                    ext if ARCHIVE_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "archives",
                        &mut log_string,
                    )?,

                    ext if DATA_INTERCHANGE_FORMAT_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "data_interchange",
                        &mut log_string,
                    )?,

                    ext if AUDIO_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "audio",
                        &mut log_string,
                    )?,

                    ext if VIDEO_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "videos",
                        &mut log_string,
                    )?,

                    ext if SOURCE_CODE_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "source_code",
                        &mut log_string,
                    )?,

                    ext if PRESENTATION_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "presentations",
                        &mut log_string,
                    )?,

                    ext if FONT_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "fonts",
                        &mut log_string,
                    )?,

                    ext if EBOOK_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "ebooks",
                        &mut log_string,
                    )?,

                    ext if EXECUTABLE_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "executables",
                        &mut log_string,
                    )?,

                    ext if DATABASE_EXTENSIONS.contains(&ext) => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "databases",
                        &mut log_string,
                    )?,

                    _ => change_file_path(
                        entry,
                        directory_to_organize.as_ref(),
                        "others",
                        &mut log_string,
                    )?,
                }
            }

            log_string.push("\n");
            logger_file.write_all(log_string.as_encoded_bytes())?;
        }
    }

    Ok(())
}

fn perform_cleanup_with_exceptions<F: AsRef<Path>>(
    entry: DirEntry,
    directory_to_organize: F,
) -> Result<(), Box<dyn Error>> {
    let entry_path = entry.path();
    let entry_path = entry_path
        .file_name()
        .ok_or("Could not retrieve file name")?;

    let mut special_folders = SPECIAL_FOLDERS
        .iter()
        .map(OsStr::new)
        .collect::<Vec<&OsStr>>();
    special_folders.push(directory_to_organize.as_ref().as_os_str());

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
    log_string: &mut OsString,
) -> Result<(), Box<dyn Error>> {
    let mut log_string_slice = new_folder_name
        .to_uppercase()
        .split('_')
        .collect::<Vec<&str>>()
        .join("-");
    log_string_slice.insert_str(0, " - ");
    log_string.push(log_string_slice);

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
