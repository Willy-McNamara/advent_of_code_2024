use std::collections::HashMap;
use std::num::ParseIntError;

pub fn middle_num(update: &Vec<&str>) -> Result<i32, ParseIntError> {
    let mid_index = (update.len() - 1) / 2;

    let mid_str = update[mid_index];
    mid_str.parse()
}

pub fn vec_from_line(line: &str) -> Vec<&str> {
    line.split(",").collect()
}

pub fn create_rules_map(rules_raw: &str) -> HashMap<&str, Vec<&str>> {
    let mut rules_map: HashMap<&str, Vec<&str>> = HashMap::new();
    
    for line in rules_raw.lines() {
        let rule_pair: Vec<&str> = line.split("|").collect();
        rules_map.entry(rule_pair[1])
            .or_insert_with(Vec::new)
            .push(rule_pair[0]);
    }

    rules_map
}


pub fn valid_update(update: &Vec<&str>, rules: &HashMap<&str, Vec<&str>>) -> bool {
    let mut off_limits_map: HashMap<&str, bool> = HashMap::new();

    for item in update.iter() {
        // Check if offlimits
        if off_limits_map.contains_key(item) {
            return false;
        } 

        // Add new offlimits nums
        if let Some(rules) = rules.get(item) {
            for rule in rules.iter() {
                off_limits_map.insert(rule, true);
            }
        } else {
            continue;
        }
    }
    true
}

pub fn sum_valid_updates(rules_file: &str, updates_file:&str) -> i32 {
    let mut valid_sum = 0;

    let rules_map = create_rules_map(rules_file);

    for update in updates_file.lines() {
        let update_vec = vec_from_line(update);
        if valid_update(&update_vec, &rules_map) {
            valid_sum += middle_num(&update_vec).unwrap();
        }
    }

    valid_sum
}




#[cfg(test)]
mod tests {
    use super::*;

    const test_update: &str = "75,47,61,53,29";
    const test_update_invalid: &str = "53,29,75,61";

    const test_rules: &str = "75|53
75|61
97|29";

    const int_test_rules: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";

    const int_test_updates: &str = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn splits_str_to_vec() {
        let mut solution_vec = vec!["75", "47", "61", "53", "29"];

        assert_eq!(solution_vec, vec_from_line(&test_update));
    }

    #[test]
    fn gets_middle_num() {
        let test_vec = vec_from_line(&test_update);
        let solution = 61;

        assert_eq!(Ok(solution), middle_num(&test_vec));
    }

    #[test]
    fn makes_rules_map() {
        let mut rules_map = HashMap::new();
        rules_map.insert("53", vec!["75"]);
        rules_map.insert("61", vec!["75"]);
        rules_map.insert("29", vec!["97"]);

        assert_eq!(rules_map, create_rules_map(test_rules));
    }

    #[test]
    fn validates_update() {
        let rules = create_rules_map(test_rules);
        let update_vec = vec_from_line(&test_update);

        assert_eq!(true, valid_update(&update_vec, &rules));
    }

    #[test]
    fn invalidates_update() {
        let rules = create_rules_map(test_rules);
        let update_vec = vec_from_line(&test_update_invalid);

        assert_eq!(false, valid_update(&update_vec, &rules));
    }

    #[test]
    fn int_test_one() {
        let sum = 143;

        assert_eq!(sum, sum_valid_updates(int_test_rules, int_test_updates));
    }

}
