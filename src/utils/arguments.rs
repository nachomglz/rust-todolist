use crate::errors::args_not_found::ArgsNotFound;
use std::env;

#[derive(Debug)]
pub struct Argument {
    name: String,
    description: String,
}

impl Argument {
    pub fn parse() -> Result<Vec<Self>, ArgsNotFound> {
        // Get the arguments
        let args: Vec<String> = env::args().collect();
        let mut arguments: Vec<Self> = Vec::<Self>::new();

        // Throw error if the "name" argument has not been passed
        if !args.contains(&"-n".to_owned()) && !args.contains(&"--name".to_owned()) {
            return Err(ArgsNotFound::new(
                "Argument \"-n\" | \"--name\" was not passed to the command!",
            ));
        }

        for (i, arg) in args.iter().enumerate() {
            // If the argument is "name", the value is going to be the next argument in the args vector
            if arg == "-n" || arg == "--name" {
                arguments.push(Argument {
                    name: "name".to_string(),
                    description: match args.get(i + 1) {
                        Some(value) => value.to_owned(),
                        _ => {
                            return Err(ArgsNotFound::new(
                                "No value was given to the \"-n\" | \"--name\" argument",
                            ))
                        }
                    },
                })
            } else if arg == "-d" || arg == "--description" {
                arguments.push(Argument {
                    name: "description".to_string(),
                    description: match args.get(i + 1) {
                        Some(value) => value.to_owned(),
                        _ => {
                            return Err(ArgsNotFound::new(
                                "No value was given to the \"-d\" | \"--description\" argument",
                            ))
                        }
                    },
                })
            }
        }

        Ok(arguments)
    }
}
