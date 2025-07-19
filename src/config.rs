use std::env;
use std::error::Error;
use std::fs::{self, File, OpenOptions, ReadDir};
use std::path::PathBuf;

// -- Program's config structure --
pub struct Config {
    pub directory_to_organize: PathBuf,
    pub root_iterator: ReadDir,
    pub logger_file: File,
}

impl Config {
    // -- Method to parse the given arguments --
    pub fn parse(args: env::Args) -> Result<Self, Box<dyn Error>> {
        let mut args = args.skip(1);

        let directory_to_organize = PathBuf::from(
            args.next()
                .ok_or("Directory to organize not provided: -- <DIRECTORY> --")?,
        );

        let logger_file_path = PathBuf::from(
            args.next()
                .ok_or("File to log not provided: -- <LOGGER-FILE> --")?,
        );

        if args.next().is_some() {
            return Err("Too many arguments provided".into());
        }

        let root_iterator = fs::read_dir(&directory_to_organize)?;
        let logger_file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(logger_file_path)?;

        Ok(Self {
            directory_to_organize,
            root_iterator,
            logger_file,
        })
    }
}
