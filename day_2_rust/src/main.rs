use std::fs;
use day_2_rust::run_parse_and_test;

fn main() {
    let file_path = "input.txt";
    let file = fs::read_to_string(file_path)
        .expect("read file in as string");
    let valid_result_count = run_parse_and_test(&file);

    println!("There are {} valid results", valid_result_count);
}
