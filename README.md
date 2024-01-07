# A mini todo-cli

> Documentation available at [docs.rs](https://docs.rs/todo-cli-manikya/0.1.0/todo_cli_manikya/)

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

### Begin TUI

Run without any arguments to start the TUI

```bash
todo-cli
```

### Help

Get help regarding cli

```bash
todo-cli help
```

You can also run help on any subcommand, e.g.

```bash
todo-cli list help
```

### List

List out all the tasks

```bash
todo-cli list
```

List out all the tasks which are not yet completed

```bash
todo-cli list -p true   # p stands for pending
```

List out all the tasks which are completed

```bash
todo-cli list -c true
```

Find out tasks using fuzzy search

```bash
todo-cli list -f "buy"    # list all tasks with buy
```

You cna combine multiple flags for more specific searches

```bash
todo-cli list -p true -f "cmp"    # all pending tasks with fuzzy
                                  # e.g. "organize computer" task
                                  # which is pending will match
```

### Clear all data

```bash
todo-cli clean
```

### Edit a task

Currently, only TUI supports editing a task but CLI feature will be added soon!

## How its made

The todo-cli follows a very simple approach, keep all the tasks in a csv and access them efficiently as and when needed.

This project stand on the shoulder of giants, by managing majority of requirements from external dependencies.

## Scope for improvement

This project is far from complete yet and needs many improvements

- [x] Improve performance while navigation the tasks in tui
- [x] Add a hash value to tasks and store then properly in csv for fast access and management
- [ ] Add options to use only CLI for adding and managing tasks
- [ ] Use hashing algorithm instead of RNG for task ID

## Missing features / Bugs

- Tasks cannot yet be edited using CLI
- Excess tasks will overflow from TUI
- No method yet to know about ID or status of a task in TUI
- Tasks are rendered naively in TUI which does not offer customization.
- No provision for storing dates when tasks are created/edited

## Some ambitious features

This list includes those features which are not currently high priority but can improve User Experience

- Allow customization of themes using yml/toml
- Make tasks richer by allowing bold, italic, highlight, internal and external links
- Tags for tasks which follow common ideas
- Extra status for tasks in-progress
