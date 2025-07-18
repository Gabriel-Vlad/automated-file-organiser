# **AUTOMATED-FILE-ORGANIZER**

This tool organizes a given directory by it's contained files' extensions.
The files are moved into specific newly created directories depending on
the name of the extension.

## The features of the software are

- Recursive Scanning
- Detailed Logging of the Files
- Flexible Usage
- High-Performance File Operation
- Automated Cleanup of Empty Folders

## How to get the software ready

- You need rust toolchain installed
- To get the code visit [Automated File Organizer](https://github.com/Gabriel-Vlad/automated-file-organiser)
- The command line to build the project: **cargo build --release**

## How to use the software

The basic command-line to run the program is:

```bash
cargo run <DIRECTORY> <LOGGER-FILE>
```

Example:

```bash
cargo run ./my_downloads cleanup_report.log
```

## Hoe does the software work

The program takes the user input, than parses it into a Config structure. After that,
the main function responsible for organizing the directory, is called. It
recursively traverses each directory and moves the files within in newly
created directories, if they don't exist already, depending on their extensions.
The moved files' paths are logged in the logger file chose by the user within the input.
After a directory is fully traversed, the program performs a cleanup and removes it.

The software's folder contains an **example** directory that has a before and after
tests showing how the software works practically.

## List of feature ideas

- Being able to chose if the program should perform a cleanup or not
