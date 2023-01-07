use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub description: String,
}

impl Task {
    pub fn show_task(&self) {
        println!(
            "Name: {n}, description: {d}",
            n = self.name,
            d = self.description
        )
    }
}

pub fn create_task(task_name: &str) {
    println!("Creating task with the name: {}", task_name);
}
pub fn remove_task(task_name: &str) {
    println!("Removing task with the name: {}", task_name);
}
pub fn modify_task(task_name: &str) {
    println!("Modifying task with the name: {}", task_name);
}
