# rust-file-explorer

A lightweight and interactive command-line file explorer built in Rust. This tool allows users to navigate directories, view files, and perform basic file operations with an intuitive interface inspired by modern CLI tools.

## Features

- **Directory Navigation**: Change directories using the `cd` command.
- **File Listing**: View the contents of the current directory with color-coded output (directories in blue, files in yellow).
- **File Operations**:
  - Create new files with `create `.
  - Delete files or directories with `delete `.
  - Open files with the default application using `open `.
- **Interactive Interface**:
  - Command prompt shows the current directory.
  - User-friendly error and success messages.
  - Color-coded output for better readability.
- **Help Menu**: Type `help` to see all available commands.

## Installation

1. Clone the repository:
   ```
    git clone https://github.com/axpico/rust-file-explorer.git
    cd rust-file-explorer
   ```

2. Install dependencies:
   Ensure you have [Rust](https://www.rust-lang.org/) installed. Then, add the required crates:
   ```
    cargo build
   ```

3. Run the program:
   ```
    cargo run
   ```

## Usage

Once the program is running, you can use the following commands:

### Commands

| Command   | Description                                              |
|-----------|----------------------------------------------------------|
| `cd `     | Change to the specified directory.                       |
| `ls`      | List all files and directories in the current directory. |
| `open `   | Open a file with the default application.                |
| `delete ` | Delete a specified file or directory.                    |
| `create ` | Create a new file in the current directory.              |
| `help`    | Show a list of available commands.                       |
| `exit`    | Exit the program.                                        |

### Example Usage

```
> ls
📂 Contents of /home/user/projects:
- src/
- README.md
- Cargo.toml

> cd src
✓ Changed directory to /home/user/projects/src

> create example.txt
✓ Created example.txt

> ls
📂 Contents of /home/user/projects/src:
- main.rs
- example.txt

> open example.txt
✓ Opened example.txt

> delete example.txt
✓ Deleted example.txt

> exit
```

## Dependencies

This project uses the following crates:

- [`anyhow`](https://crates.io/crates/anyhow): Simplified error handling with context.
- [`rustyline`](https://crates.io/crates/rustyline): Interactive command-line editor.
- [`colored`](https://crates.io/crates/colored): Terminal text styling and coloring.
- [`open`](https://crates.io/crates/open): Cross-platform file opener.

Add them to your `Cargo.toml`:

```
[dependencies]
anyhow = "1.0"
rustyline = "15.0.0"
open = "5.3.2"
colored = "3.0.0"
```

## License

This project is licensed under the [MIT License](LICENSE).

---

Happy exploring! 🚀
