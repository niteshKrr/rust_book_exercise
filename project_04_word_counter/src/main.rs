use std::fs::File;
use std::io::{self, Read};

fn string_file() -> Result<String, io::Error> {
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

fn count_words(items: &str) -> usize {
    items.split_whitespace().count()
}

fn count_chars(items: &str) -> usize {
    items.len()
}

fn count_lines(items: &str) -> usize {

    let mut line_count = 0;
    for _ in items.lines() {
        line_count += 1;
    }

    line_count
}

fn main() {
    let item_in_file = string_file();
    let items = match item_in_file {
        Ok(something) => something,
        Err(_err) => panic!("not gettin anything"),
    };

    let total_words = count_words(&items);
    let total_char = count_chars(&items);
    let total_lines = count_lines(&items);


    println!("total words in given file is: {}", total_words);
    println!("total chars in given file is: {}", total_char);
    println!("total lines in given file is: {}", total_lines);
}
