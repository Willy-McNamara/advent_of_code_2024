use std::fs;
use Day_1::run_parse;

fn main() {
    let file_path = "input.txt";
    let file = fs::read_to_string(file_path)
	.expect("should be able to read in file");    

    let result: i32 = run_parse(&file);
    println!("The total difference is {}", result);
}
