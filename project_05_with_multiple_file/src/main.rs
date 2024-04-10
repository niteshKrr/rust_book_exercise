mod extracting_all_string;
use extracting_all_string::string_file;
use extracting_all_string::{count_total_words,count_total_chars as t_char,count_total_lines};

fn main() {
    let item_in_file = string_file();
    let items = match item_in_file {
        Ok(something) => something,
        Err(_err) => panic!("not gettin anything"),
    };

    let total_words = count_total_words::count_words(&items);
    let total_char = t_char::count_chars(&items);
    let total_lines = count_total_lines::count_lines(&items);

    println!("total words in given file is: {}", total_words);
    println!("total chars in given file is: {}", total_char);
    println!("total lines in given file is: {}", total_lines);
}
