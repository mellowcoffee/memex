#![allow(clippy::needless_for_each)]

use crate::{
    files::{FILES_DIR_PATH, read_files_from_dir},
    model::Wiki,
};

mod error;
mod files;
mod model;
mod parser;

fn main() {
    let files = read_files_from_dir(FILES_DIR_PATH).unwrap();
    let wiki = Wiki::from_files(files).unwrap();
    println!("{wiki:#?}");
}
