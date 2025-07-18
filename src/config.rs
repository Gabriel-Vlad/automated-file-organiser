use std::env::Args;
use std::error::Error;
use std::fs::{self, File, OpenOptions, ReadDir};
use std::path::PathBuf;

// -- Program's config structure --
pub struct Config {
    pub directory_to_sort: PathBuf,
    pub root_iterator: ReadDir,
    pub files_log: File,
}

impl Config {
    // -- Method to parse the given arguments --
    pub fn parse(args: Args) -> Result<Self, Box<dyn Error>> {
        let mut args = args.skip(1);

        let directory_to_sort = PathBuf::from(
            args.next()
                .ok_or("Document to sort not provided: -- <directory-to-sort> --")?,
        );

        let files_log_path = PathBuf::from(
            args.next()
                .ok_or("File to log not provided: -- <log-file> --")?,
        );

        if args.next().is_some() {
            return Err("Too many arguments provided".into());
        }

        let root_iterator = fs::read_dir(&directory_to_sort)?;
        let files_log = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(files_log_path)?;

        Ok(Self {
            directory_to_sort,
            root_iterator,
            files_log,
        })
    }
}
