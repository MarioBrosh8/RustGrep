# minigrep

A command-line tool implemented in Rust for searching for a specific string within a file, similar to the `grep` utility.

This project is inspired by the *The Rust Programming Language* book's "minigrep" tutorial.

## Features

- **Efficient String Searching**: Filters lines in a specified file that contain a given search query.
- **Error Handling**: Graceful error handling for missing arguments or file reading issues.

## Getting Started

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your machine.

### Installation

Clone the repository and build the project:

```bash
git clone <repository-url>
cd minigrep
cargo build --release
```

## Usage

To use `minigrep`, run it through `cargo run` with a search query and a file path as arguments:

```bash
cargo run -- <query> <file_path>
```

### Example

Suppose you have a file named `sample.txt` with some text. To search for the word "rust":

```bash
cargo run -- rust sample.txt
```

The program will output each line that contains the string "rust".

## Project Structure

- `src/main.rs`: Handles command-line argument parsing and configuration.
- `src/lib.rs`: Contains the core logic for searching strings in text.
- `Cargo.toml`: Project metadata and dependencies.
