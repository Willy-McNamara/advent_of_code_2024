use std::fs;

pub fn run_parse(file: &str) -> i32 {
    let mut difference: i32 = 0;    
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    let lines: Vec<&str> = parse_lines(file);
    for line in lines {
	let parts: Vec<&str> =  parse_line(line);
	list_one.push(str_to_number(parts[0]));
        list_two.push(str_to_number(parts[1]));
    }

    list_one.sort();
    list_two.sort();

    let mut index: usize = 0;
    for num in list_one {
	difference += find_difference(&num, &list_two[index]); 
	index += 1;
    }
    difference
}

pub fn find_difference(number_one: &i32, number_two: &i32) -> i32 {
    if number_one > number_two {
	number_one - number_two
    } else {
	number_two - number_one
    }
}

pub fn parse_lines(lines: &str) -> Vec<&str> {
    let mut lines_vec: Vec<&str> = Vec::new();
    for line in lines.lines() {
	lines_vec.push(line);
    }
    lines_vec
}

pub fn parse_line(line: &str) -> Vec<&str> {
    let mut line_parts_vec: Vec<&str> = Vec::new();
    for part in line.split_whitespace() {
	line_parts_vec.push(part);
    }
    line_parts_vec
}

pub fn str_to_number(str: &str) -> i32 {
    let num: i32 = str.parse().expect("not valid");
    num 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difference_of_three() {
	let number_one = 10004;
	let number_two = 10001;

	let difference = 3;
	assert_eq!(difference, find_difference(&number_one, &number_two));
    }

    #[test]
    fn reads_two_lines() {
	let lines = "line one
line two";
	
	let mut lines_vec: Vec<&str> = Vec::new();
	lines_vec.push("line one");
	lines_vec.push("line two"); 

	assert_eq!(lines_vec, parse_lines(lines));
    }

    #[test]
    fn parses_one_line() {
	let line = "10001    10002";
	let mut line_parts_vec: Vec<&str> = Vec::new();
	line_parts_vec.push("10001");
	line_parts_vec.push("10002");

	assert_eq!(line_parts_vec, parse_line(line));
    }

    #[test]
    fn parses_string_to_number() {
	let string = "10001";
	let int = 10001;

	assert_eq!(int, str_to_number(string));
    }

    #[test]
    fn finds_total_diff() {
	let dummy_file = "10001    10002
10003    10004
10006    10005";
	let total_diff = 3;
	
	assert_eq!(total_diff, run_parse(dummy_file)); 
    }
}


