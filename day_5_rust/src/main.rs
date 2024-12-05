use std::fs;
use day_5_rust::sum_valid_updates;

fn main() {
    let rules_filename = "rules_input.txt";
    let updates_filename = "updates_input.txt";

    let rules = fs::read_to_string(rules_filename)
        .expect("reads in rules");
    let updates = fs::read_to_string(updates_filename)
        .expect("reads in updates");

    let sum = sum_valid_updates(&rules, &updates);

    println!("There are {} valid updates", sum);
}
