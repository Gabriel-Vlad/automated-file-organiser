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
the function responsible for organizing the directory is called. It
recursively traverses each directory and moves the files within in newly
created directories with suggestive names, if they don't exist already, depending on their extensions.
For seeing the names that this software uses for its directories please go to **src/organizer/constants.rs**.
The moved files' paths are logged in the logger file chose by the user within the input.
After a directory is fully traversed, the program performs a cleanup and removes it.

The software's folder contains an **example** directory that has a before and after
tests showing how the software works practically.

## How to use environment variables

Setting on Linux and MacOS:

```bash
MY_VAR=my-secret-key""
```

Setting on Windows - Powershell:

```bash
$env:MY_VAR="my-secret-key"
```

Setting on Windows - CMD:

```bash
set MY_VAR="my-secret-key"
```

## How to unset environment variables

A variable must exist in order to be unset,
otherwise there will be an error.

Unsetting on Linux and MacOS:

```bash
echo $MY_VAR
unset MY_VAR
```

Unsetting on Windows - Powershell:

```bash
Write-Host $env:MY_VAR
Remove-Item env:MY_VAR
```

Unsetting on Windows - CMD:

```bash
echo %MY_VAR%
set MY_VAR=
```

The functions **echo $MY_VAR**, **Write-Host $env:MY_VAR**, **echo %MY_VAR%**
will print to the console the value of **MY_VAR** if it exists, otherwise
will print nothing.

## Choosing if the program should perform a cleanup

The environment variable name is **CLEANUP**. This can only be set to **true** or **false**.
Any other values will result in an **error** terminating the process early. The variable is
set to **false** by default.The value **true** indicates that the program should perform a
cleanup, whereas the value **false** indicated that it should not perform a cleanup.

> Example

```bash
$env:CLEANUP="true"
Write-Host $env:CLEANUP
Remove-Item env:CLEANUP
```
