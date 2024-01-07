use clap::{Args as ClapArgs, Parser, Subcommand};
use cli_table::{print_stdout, Cell, Color, Style, Table};

use crate::{
    files::{
        check_existing_metadata, create_metadata, enter_data_to_file, read_data_from_file,
        remove_metadata,
    },
    tui::run,
    Id, Result,
};

/// Args to be used for the application
#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// All the available commands
#[derive(Subcommand)]
enum Commands {
    /// Remove all the existing tasks from the database
    Clean,
    /// List out all the existing tasks
    List(ListArgs),
    /// Add a new task
    Add(AddArgs),
    /// Remove a task
    Remove(RemoveArgs),
}

#[derive(ClapArgs)]
struct ListArgs {
    /// List only the tasks which are completed
    #[arg(short)]
    completed: Option<bool>,
    /// List only the tasks which are incomplete
    #[arg(short = 'n')]
    incomplete: Option<bool>,
    /// Show a task with particular id
    #[arg(short)]
    id: Option<Id>,
}

#[derive(ClapArgs)]
struct AddArgs {
    /// Description for the command to be added
    #[arg(short)]
    description: String,
}

#[derive(ClapArgs)]
struct RemoveArgs {
    /// Id of the task to be removed
    #[arg(short)]
    id: Id,
}

impl Args {
    /// The main function which runs the entire app
    pub fn run(&self) -> Result<()> {
        if let Some(command) = &self.command {
            match command {
                Commands::Clean => {
                    println!("Are you sure you want to delete all tasks?(y/n)");
                    let mut ans = String::new();
                    std::io::stdin().read_line(&mut ans)?;
                    let ans = ans.trim();
                    if ans.eq("y") {
                        remove_metadata()?;
                        println!("All tasks removed successfuly");
                    }
                }
                Commands::List(options) => {
                    if check_existing_metadata() {
                        let data = read_data_from_file()?;
                        if data.tasks.is_empty() {
                            println!("No tasks yet!");
                        } else {
                            // only want a certain task
                            if let Some(id) = options.id {
                                let task = data.tasks.get(&id);
                                if let Some(task) = task {
                                    println!(
                                        "TASK FOUND\nDescription: {}\nStatus: {}",
                                        task.desc,
                                        match task.completed {
                                            true => "Completed",
                                            false => "Pending",
                                        }
                                    );
                                } else {
                                    println!("No such task found!");
                                }
                            // print multiple tasks in a table
                            } else {
                                let mut table = Vec::new();
                                for task in data.get_tasks() {
                                    if (options.completed.is_some() && !task.completed)
                                        || (options.completed.is_none() && task.completed)
                                    {
                                        continue;
                                    }
                                    table.push(vec![
                                        task.id.cell(),
                                        task.desc.clone().cell(),
                                        match task.completed {
                                            true => "Completed"
                                                .cell()
                                                .foreground_color(Some(Color::Green)),
                                            false => "Pending"
                                                .cell()
                                                .bold(true)
                                                .foreground_color(Some(Color::Red)),
                                        },
                                    ]);
                                }
                                let table = table.table().title(vec![
                                    "Task ID"
                                        .cell()
                                        .bold(true)
                                        .foreground_color(Some(Color::Blue)),
                                    "Task Description"
                                        .cell()
                                        .bold(true)
                                        .foreground_color(Some(Color::Blue)),
                                    "Status"
                                        .cell()
                                        .bold(true)
                                        .foreground_color(Some(Color::Blue)),
                                ]);
                                print_stdout(table)?;
                            }
                        }
                    } else {
                        println!("No tasks yet");
                    }
                }
                Commands::Add(add_args) => {
                    if !check_existing_metadata() {
                        create_metadata()?;
                    }
                    let mut data = read_data_from_file()?;
                    data.add_task(&add_args.description);
                    enter_data_to_file(&data)?;
                    println!("Added new task successfully");
                }
                Commands::Remove(remove_args) => {
                    if check_existing_metadata() {
                        let mut data = read_data_from_file()?;
                        if data.remove_task(&remove_args.id).is_none() {
                            println!("No such task found");
                        } else {
                            enter_data_to_file(&data)?;
                        }
                    }
                }
            }
        } else {
            run()?;
        }
        Ok(())
    }
}
