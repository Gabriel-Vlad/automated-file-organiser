use std::error::Error;
use std::fs::{self, DirBuilder, ReadDir};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::constants::MESSY_DIR;

mod constants {
    pub static MESSY_DIR: &'static str = "messy_folder";
}

fn parse_entries<F: AsRef<Path>>(root: ReadDir, main_folder: F) -> Result<(), Box<dyn Error>> {
    for entry in root {
        let entry = entry?;
        let file_metadata = entry.metadata()?;

        if file_metadata.is_dir() {
            let new_root = fs::read_dir(entry.path())?;
            parse_entries(new_root, main_folder.as_ref())?;

            fs::remove_dir(entry.path())?;
        } else {
            print!("{:?}", entry.path());

            if let Some(ext) = entry.path().extension() {
                let mut dir_builder = DirBuilder::new();
                dir_builder.recursive(true);

                let mut new_path: PathBuf = PathBuf::new();

                match ext.to_str().ok_or("Something went wrong")? {
                    "png" | "jpeg" | "jpg" => {
                        print!("- image");

                        let images_dir = main_folder.as_ref().join("images");
                        dir_builder.create(&images_dir)?;

                        if let Some(file_name) = entry.path().file_name() {
                            new_path = new_path.join(images_dir).join(file_name);
                        }

                        fs::rename(entry.path(), new_path)?;
                    }

                    "pdf" | "txt" => {
                        print!("- document");

                        let documents_dir = main_folder.as_ref().join("documents");
                        dir_builder.create(&documents_dir)?;

                        if let Some(file_name) = entry.path().file_name() {
                            new_path = new_path.join(documents_dir).join(file_name);
                        }

                        fs::rename(entry.path(), new_path)?;
                    }

                    "zip" => {
                        print!("- archive");

                        let archive_dir = main_folder.as_ref().join("archives");
                        dir_builder.create(&archive_dir)?;

                        if let Some(file_name) = entry.path().file_name() {
                            new_path = new_path.join(archive_dir).join(file_name);
                        }

                        fs::rename(entry.path(), new_path)?;
                    }

                    _ => print!("- random"),
                }
            }

            print!("\n");
            std::io::stdout().flush().unwrap();
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let root: ReadDir = fs::read_dir(MESSY_DIR)?;
    let main_folder = Path::new(MESSY_DIR).to_owned();
    parse_entries(root, main_folder)?;
    Ok(())
}
