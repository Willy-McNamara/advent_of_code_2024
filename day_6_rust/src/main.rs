use day_6_rust::run_job;
use std::fs;

fn main() {
    let filename = "src/input.txt";

    let file = fs::read_to_string(filename)
        .expect("couldn't find file");

    let total = run_job(&file);

    println!("The total distinct positions is {}", total);
}
