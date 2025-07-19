mod config;
mod organizer;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // -- Parsing input arguments --
    let mut config = config::Config::parse(env::args())?;

    let perform_cleanup = match env::var("CLEANUP") {
        Ok(value) => value.parse::<bool>()?,
        Err(_) => false,
    };

    // -- Calling the main function to organize the directory --
    organizer::parse_entries(
        config.root_iterator,
        config.directory_to_sort,
        &mut config.files_log,
        perform_cleanup,
    )?;

    println!("Selected directory successfully organized!");

    Ok(())
}
