# hashdir

`hashdir` is a command-line tool for recursively hashing files in a specified directory using various hashing algorithms
written in rust.

## Usage

### Command-line Arguments

- `path`: The directory path to read files from.
- `--quiet` or `-q`: Run in quiet mode without progress indicators. Default is `false`.
- `--algorithm` or `-a`: The hashing algorithm to use. Options are `Sha256`, `Sha512`, and `Md5`. Default is `Sha256`.

### Example Commands

```bash
# Compute SHA-256 hash for all files in the "example_directory"
hashdir path/to/example_directory

# Compute MD5 hash for all files in the "example_directory" with quiet mode
hashdir path/to/example_directory -q -a Md5
```

## What It Does

- Recursively reads all files from the specified directory.
- Computes their hash values using the specified algorithm.
- Displays progress indicators unless quiet mode is enabled.
- Outputs the final hash value after processing all files.

