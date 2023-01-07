use crate::models::task::Task;
use crate::utils::arguments::Argument;
use std::{fs, io::Write};

pub fn create_data_dir() -> bool {
    match fs::create_dir("./data") {
        Ok(_) => true,
        _ => false,
    }
}

pub fn create_data_file() {
    if let Ok(mut file) = fs::File::create("data/db.json") {
        match file.write_all("{\"tasks\":[{\"name\":\"Echar de comer a los perros\",\"description\":\"hay que comprar la comida!\"}]}".as_bytes()) {
          Err(_) => panic!("There was an error writing to the file"),
          _ => ()
        }
    } else {
        panic!("The file couldn't be created");
    }
}

pub fn add_task() {
    // Get arguments
    if let Ok(arguments) = Argument::parse() {
        // Create the new task
        let task_name = match arguments.iter().find(|p| p.name == "name") {
            Some(key) => key.description.clone(),
            None => panic!("Some error occured"),
        };

        let task_description = match arguments.iter().find(|p| p.name == "description") {
            Some(key) => key.description.clone(),
            None => panic!("Some error occured"),
        };

        let new_task: Task = Task {
            name: task_name,
            description: task_description,
        };

        println!("This is the new Task: {:?}", new_task)
    }
}
