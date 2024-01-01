# A mini todo-cli

## About

This is a small todo app written in rust for learning purposes.

## Features

- An easy to use TUI with explanation of all keymaps
- Tasks stored in local storage so that they can be accessed later
- High performance and no lag even on slow devices
- cli commands for advanced users who do not need a tui

## How to install

### Using binary

Download the executable for your platform from the github latest release!

### Using cargo

Clone this repository using git

```bash

git clone "https://github.com/Manikya-Sharma/todo-cli"

```

cd to directory and build using Cargo

```bash
cd todo-cli
cargo build --release
```

Simply run the exetuable created in `target/release` directory

Alternatively, use cargo run while providing `opt-level=3` in `Cargo.toml`.

## Usage

Get help regarding cli

```bash
todo-cli help
```

Run without any arguments to start the TUI

```bash
todo-cli  # starts tui in alternate screen
```

List all the existing tasks in the terminal

```bash
todo-cli list
```

Remove all the existing tasks

```bash
todo-cli clean
```

### Navigating in TUI

You can use either the arrow keys or `j` and `k` to move down and up respectively

## How its made

Some of the amazing crates which have been used to build this project are: -

- `ratatui` and `crossterm`
- `clap`
- `dirs`
- `csv`

Intensive documentation has been done for almost all the functions and methods to make the code easy to understand

## Scope for improvement

This project is far from complete yet and needs many improvements

- Improve performance while navigation the tasks in tui
- Add option to use only CLI for adding and managing tasks
- Add a hash value to tasks and store then properly in csv for fast access and management

## Missing features

- Tasks can only be deleted but not marked as complete
- Updating task is using poor makeshift method by replacing instead of actually updating it
