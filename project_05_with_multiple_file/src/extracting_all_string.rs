use std::fs::File;
use std::io::{self, Read};

pub fn string_file() -> Result<String, io::Error> {
    let txt_file = File::open("../hello.txt");

    let mut getting_file = match txt_file {
        Ok(file) => file,
        Err(err) => panic!("err getting in opening file {:?}", err),
    };

    let mut total_items_in_file = String::new();
    match getting_file.read_to_string(&mut total_items_in_file) {
        Ok(_) => Ok(total_items_in_file),
        Err(err) => Err(err),
    }
}

pub mod count_total_words;
pub mod count_total_chars;
pub mod count_total_lines;

