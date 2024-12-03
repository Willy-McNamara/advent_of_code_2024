use std::fs;
use day_3_rust::run_parse;

fn main() {
    let file_path = "input.txt";
    let file = fs::read_to_string(file_path)
        .expect("read file in as string");
    let sum = run_parse(&file);

    println!("The total sum is {}", sum);
}
