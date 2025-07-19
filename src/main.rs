mod config;
mod organizer;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // -- Parsing input arguments --
    let mut config = config::Config::parse(env::args())?;

    // -- This variables indicated to the program
    // if it should perform a cleanup on the directories --
    let perform_cleanup = should_perform_cleanup()?;

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

fn should_perform_cleanup() -> Result<bool, Box<dyn Error>> {
    let result = env::var("CLEANUP").map_or(Ok(false), |var| var.parse::<bool>())?;

    Ok(result)
}
