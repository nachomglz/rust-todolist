use crate::models::task::Task;
use crate::utils::arguments::Argument;
use serde_json;
use std::{
    env::current_exe,
    fs::{self, metadata},
    io::{BufReader, Write},
};

fn create_data_dir() {
    let current_binary_dir = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    match fs::create_dir_all(current_binary_dir + "/data") {
        Err(e) => panic!("Error creating data directory: {:?}", e),
        _ => (),
    }
}

pub fn create_data_file() {
    let data_dir: String = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "/data";
    let data_path: String = data_dir.clone() + "/db.json";

    if let Err(_) = metadata(&data_path) {
        // The data file does not exist
        if let Err(_) = metadata(data_dir) {
            // The data directory doest not exist
            create_data_dir()
        }
        match fs::File::create(&data_path) {
            Ok(mut file) => match file.write_all("[]".as_bytes()) {
                Err(_) => panic!("Failed to create the file"),
                _ => (),
            },
            Err(e) => {
                println!("There was an error creating the data file: {:?}", e)
            }
        }
    }
}

fn read_data() -> Vec<Task> {
    let data_path: String = get_data_file_path();
    let file = fs::File::open(data_path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn write_data(task_list: Vec<Task>) {
    let data_path = get_data_file_path();

    // Parse task list to string json
    if let Ok(json) = serde_json::to_string(&task_list) {
        println!("The new json is: {:?}", json);
        // Open data file
        if let Ok(mut file) = fs::File::create(data_path) {
            file.write_all(json.as_bytes()).unwrap();
        }
    }
}

fn get_data_file_path() -> String {
    current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "/data/db.json"
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

        let mut task_list: Vec<Task> = read_data();
        println!("The old task list is: {:?}", task_list);
        task_list.push(new_task);

        write_data(task_list);
    }
}
