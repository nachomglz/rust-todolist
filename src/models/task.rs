mod Task {
    struct Task {
        name: String,
        description: String,
    }

    fn create_task(task_name: &str) -> Result<boolean, io::Err> {
        println!("Creating task with the name: {}", task_name);
    }
    fn remove_task(task_name: &str) {
        println!("Removing task with the name: {}", task_name);
    }
    fn modify_task(task_name: &str) {
        println!("Modifying task with the name: {}", task_name);
    }
}
