use utils::arguments::Argument;

mod errors;
mod models;
mod utils;

fn main() {
    // Get arguments
    if let Ok(arguments) = Argument::parse() {
        println!("The arguments are: {:?}", arguments);
    }
}
