use std::error::Error;
use std::fmt;

// Create Error type and implement Debug trait
#[derive(Debug)]
struct ArgsNotFound {
    message: String,
}

// Implement struct and create the `new` function
impl ArgsNotFound {
    fn new(msg: &str) -> ArgsNotFound {
        ArgsNotFound {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for ArgsNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArgsNotFound Error: {}", self.message)
    }
}

impl Error for ArgsNotFound {}
