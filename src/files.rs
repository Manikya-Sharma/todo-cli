use std::{
    fs::{self, File},
    io::{Error, ErrorKind},
    path::Path,
};

use csv::ReaderBuilder;

use crate::{
    state::{State, Task},
    Result,
};

const FOLDER_NAME: &str = ".todo-cli";
const CSV_NAME: &str = ".todo-cli/data.csv";

/// Checks whether data for tasks already exists
pub fn check_existing_metadata() -> bool {
    match dirs::home_dir() {
        Some(dir) => dir.join(Path::new(FOLDER_NAME)).exists(),
        None => false,
    }
}

/// Create storage in home directory
pub fn create_metadata() -> Result<()> {
    if let Some(home) = dirs::home_dir() {
        fs::create_dir(home.join(Path::new(FOLDER_NAME)))?;
    }
    Ok(())
}

/// Serialize and enter the data to the file
pub fn enter_data_to_file(state: &State) -> Result<()> {
    if let Some(home) = dirs::home_dir() {
        let tasks_as_vec = state.get_tasks();
        let mut writer = csv::Writer::from_path(home.join(Path::new(CSV_NAME)))?;

        // header
        writer.write_record(["id", "desc", "status", "updated"])?;

        // contents
        for task in tasks_as_vec {
            writer.write_record(&[
                task.id.to_string(),
                task.desc.clone(),
                task.completed.to_string(),
                task.last_updated.to_string(),
            ])?;
        }

        writer.flush()?;
        Ok(())
    } else {
        Err(Box::new(Error::new(
            ErrorKind::NotFound,
            "No metadata found",
        )))
    }
}

/// Deserialize daat from the given file
pub fn read_data_from_file() -> Result<State> {
    if let Some(home) = dirs::home_dir() {
        let mut state = State::new();
        let file = File::open(home.join(Path::new(CSV_NAME)))?;
        let mut reader = ReaderBuilder::new().from_reader(file);
        for record in reader.records() {
            let record = record?;
            let task: Task = record.deserialize(None)?;
            let id = task.id;
            state.tasks.insert(
                id,
                crate::state::ListItem {
                    task,
                    selected: false,
                },
            );
            state.ids.push(id);
        }
        Ok(state)
    } else {
        Err(Box::new(Error::new(
            ErrorKind::NotFound,
            "No metadata found",
        )))
    }
}

/// Remove the directory which stores tasks data
///
/// User must confirm before this is being called
pub fn remove_metadata() -> Result<()> {
    match dirs::home_dir() {
        Some(home) => {
            fs::remove_dir_all(home.join(FOLDER_NAME))?;
            Ok(())
        }
        _ => Err(Box::new(Error::new(
            ErrorKind::NotFound,
            "No metadata found",
        ))),
    }
}
