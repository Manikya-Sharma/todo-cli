use clap::{Args as ClapArgs, Parser, Subcommand};
use cli_table::{print_stdout, Cell, Color, Style, Table};
use sublime_fuzzy::best_match;

use crate::{
    files::{
        check_existing_metadata, create_metadata, enter_data_to_file, read_data_from_file,
        remove_metadata,
    },
    format_date,
    state::State,
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
    /// Edit a task with an id
    Edit(EditArgs),
    /// Mark a task complete or incomplete
    Mark(MarkArgs),
}

#[derive(ClapArgs)]
struct ListArgs {
    /// List only the tasks which are completed
    #[arg(short)]
    completed: Option<bool>,
    /// List only the tasks which are incomplete
    #[arg(short = 'p')]
    incomplete: Option<bool>,
    /// Show a task with particular id
    #[arg(short)]
    id: Option<Id>,
    /// Get the required tasks using a fuzzy search
    #[arg(short = 'f')]
    fuzzy: Option<String>,
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

#[derive(ClapArgs)]
struct EditArgs {
    #[arg(short)]
    id: Id,
    #[arg(short)]
    description: String,
}

#[derive(ClapArgs)]
struct MarkArgs {
    #[arg(short)]
    id: Id,
}

fn show_multiple_tasks_in_a_table(data: State, options: &ListArgs) -> Result<()> {
    let mut table = Vec::new();
    for task in data.get_tasks() {
        if (options.completed.is_some() && !task.completed)
            || (options.incomplete.is_some() && task.completed)
        {
            continue;
        }
        // fuzzy search
        if let Some(search) = &options.fuzzy {
            if best_match(search, &task.desc).is_none() {
                continue;
            }
        }
        table.push(vec![
            task.id.cell(),
            task.desc.clone().cell(),
            match task.completed {
                true => "Completed".cell().foreground_color(Some(Color::Green)),
                false => "Pending"
                    .cell()
                    .bold(true)
                    .foreground_color(Some(Color::Red)),
            },
            format_date(task.last_updated).cell(),
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
        "Last Updated"
            .cell()
            .bold(true)
            .foreground_color(Some(Color::Blue)),
    ]);
    print_stdout(table)?;
    Ok(())
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
                                if let Some(list_item) = task {
                                    println!(
                                        "TASK FOUND\nDescription: {}\nStatus: {}",
                                        list_item.task.desc,
                                        match list_item.task.completed {
                                            true => "Completed",
                                            false => "Pending",
                                        }
                                    );
                                } else {
                                    println!("No such task found!");
                                }
                            } else {
                                show_multiple_tasks_in_a_table(data, options)?;
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
                Commands::Edit(edit_args) => {
                    if check_existing_metadata() {
                        let mut data = read_data_from_file()?;
                        if data.remove_task(&edit_args.id).is_some() {
                            data.add_task(&edit_args.description);
                            enter_data_to_file(&data)?;
                            println!("Task changed successfully");
                        } else {
                            println!("No task with this id found");
                        }
                    }
                }
                Commands::Mark(mark_args) => {
                    if check_existing_metadata() {
                        let mut data = read_data_from_file()?;
                        if let Some(complete) = data.toggle_task_status_by_id(mark_args.id) {
                            enter_data_to_file(&data)?;
                            if complete {
                                println!("Marked task as complete");
                            } else {
                                println!("Marked task as incomplete");
                            }
                        } else {
                            println!("No such task found");
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
