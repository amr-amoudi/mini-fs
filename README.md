# mini-fs

A minimal file system simulator built in Rust. Navigate and manipulate a tree-structured virtual file system through a simple shell-like interface.

Built as a learning project to explore core Rust concepts like ownership, smart pointers, and interior mutability.

## Features

- Tree-structured virtual file system
- Shell-like command interface
- Navigate directories with `cd` (supports absolute paths, relative paths, and `..`)
- Print current location with `pwd`
- Create new directories with `mkdir`

## Commands

| Command        | Description                                                           |
|----------------|-----------------------------------------------------------------------|
| `cd <path>`    | Change directory (e.g. `cd /home`, `cd ./Desktop`, `cd ..`, `cd home`) |
| `pwd`          | Print current directory                                               |
| `mkdir <name>` | Create a new directory in the current location                        |
| `ls`           | Print current items in the direcotry                                  |                       |
| `touch`        | Create new files (e.g. text.txt)                                   |
## Project Structure

```
src/
├── main.rs
├── dirs/
│   └── Dir.rs        # Dir struct
├── files/
│   └── file.rs       # File struct
└── file_system/
    ├── content.rs    # Content enum (Dir | File)
    ├── system.rs     # System — manages current state and commands
    └── commands/
        ├── parse_command.rs  # Command parsing
        ├── input.rs  # Handle user input parsing
        └── path.rs # Handles navigating through the system
```

## Rust Concepts Explored

- Ownership and borrowing
- `Rc<T>` for multiple ownership
- `RefCell<T>` for interior mutability
- `Weak<T>` for parent references without reference cycles
- Enums and pattern matching
- Traits
- Lifetimes

## Getting Started

```bash
git clone https://github.com/yourusername/mini-fs
cd mini-fs
cargo run
```

## Example

```
$ cargo run
>> mkdir docs
>> cd /docs
>> pwd
/docs
>> cd ..
>> pwd
/
```
