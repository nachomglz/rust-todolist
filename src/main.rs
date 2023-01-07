use std::{fs, io::Write};
use utils::arguments::Argument;
use utils::functions::{add_task, create_data_dir, create_data_file};

mod errors;
mod models;
mod utils;

fn main() {
    create_data_dir();
    create_data_file();
    add_task();
}
