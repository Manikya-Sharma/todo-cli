use clap::{Parser, Subcommand};

use crate::{
    files::{check_existing_metadata, read_data_from_file, remove_metadata},
    tui::run,
    Result,
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
    List,
}

impl Args {
    /// The main function which runs the entire app
    pub fn run(&self) -> Result<()> {
        if let Some(command) = &self.command {
            match command {
                Commands::Clean => {
                    remove_metadata()?;
                }
                Commands::List => {
                    if check_existing_metadata() {
                        let data = read_data_from_file()?;
                        if data.tasks.is_empty() {
                            println!("No tasks yet");
                        } else {
                            for task in data.get_tasks() {
                                println!(" == {}", task.desc)
                            }
                        }
                    } else {
                        println!("No tasks yet");
                    }
                }
            }
        } else {
            run()?;
        }
        Ok(())
    }
}
