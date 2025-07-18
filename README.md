# **AUTOMATED-FILE-ORGANIZER**

This tool organizes a given directory by it's contained files' extension.
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
