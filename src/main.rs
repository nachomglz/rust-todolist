use std::env;

fn main() {
    // Get the name of the task passed to the command
    let args: Vec<String> = env::args().collect();
    let task_name: &str = match args.get(1) {
        Some(name) => name,
        _ => panic!("The argument name was not provided"),
    };

    // Once we have the new task's name, we have to create the task

    // The arg 1 is the name of the binary, so we have to get the arg 2 to get the name
}
