
pub fn jump_size<'a>(curr: &'a i16, next: &'a i16) -> i16 {
    if curr > next {
        curr - next
    } else {
        next - curr
    }
}

pub fn is_safe_jump_size(jump_size: i16) -> bool {
    let too_big = jump_size > 3;
    let too_small = jump_size < 1;

    !too_big && !too_small
}

pub fn is_asc<'a>(curr: &'a i16, next: &'a i16) -> bool {
    next > curr
}

pub fn jump_direction_is_stable<'a>(original_direction: &'a bool, curr_direction: &'a bool) -> bool {
    original_direction == curr_direction
}

pub fn parse_line(line: &str) -> Vec<i16> {
    let mut int_vec = Vec::<i16>::new();
    let split_line = line.split_whitespace();
    for item in split_line {
        int_vec.push(item.parse().expect("valid input to change string to int"));
    }
    
    int_vec
}

pub fn is_safe_line(line: &Vec<i16>) -> bool {
    let direction = is_asc(&line[0], &line[1]);
    let mut result = true;
    
    for i in 0..line.len() - 1 {
        let curr_num = &line[i];
        let next_num = &line[i + 1];
        
        let stable_direction = jump_direction_is_stable(&direction, &is_asc(&curr_num, next_num));
        let safe_jump = is_safe_jump_size(jump_size(&curr_num, next_num));
        
        if !stable_direction || !safe_jump {
            result = false;
            break;
        }
    }
    result
}

pub fn parse_lines(lines: &str) -> Vec<&str> {
    let mut lines_vec: Vec<&str> = Vec::new();
    for line in lines.lines() {
	lines_vec.push(line);
    }
    lines_vec
}

pub fn run_parse_and_test(file: &str) -> i16 {
    let mut result_counter: i16 = 0;

    let lines: Vec<&str> = parse_lines(file);
    for line in lines {
        if is_safe_line(&parse_line(line)) {
            result_counter += 1;
        }
    }
    result_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_size_same() {
        let current_int = 3;
        let next_int = 3;
        let jump = 0;

        assert_eq!(jump, jump_size(&current_int, &next_int));
    }
    

    #[test]
    fn jump_size_asc() {
        let current_int = 1;
        let next_int = 3;
        let jump = 2;

        assert_eq!(jump, jump_size(&current_int, &next_int));
    }

    #[test]
    fn jump_size_desc() {
        let current_int = 10;
        let next_int = 3;
        let jump = 7;

        assert_eq!(jump, jump_size(&current_int, &next_int));
    }
   
    #[test]
    fn safe_jump() {
	let jump_size  = 1;
        let is_safe = true;

        assert_eq!(is_safe, is_safe_jump_size(jump_size));
    }

    #[test]
    fn unsafe_jump() {
        let jump_size  = 4;
        let is_safe = false;

        assert_eq!(is_safe, is_safe_jump_size(jump_size));
    }

    #[test]
    fn jump_is_ascending(){
        let curr_int = 1;
        let next_int = 3;
        let is_ascending = true;

        assert_eq!(is_ascending, is_asc(&curr_int, &next_int));
    }

    #[test]
    fn jump_is_descending() {
        let curr_int = 5;
        let next_int = 1;
        let is_ascending = false;

        assert_eq!(is_ascending, is_asc(&curr_int, &next_int));
    }

    #[test]
    fn jump_is_unstable() {
        let original_jump = false;
        let current_jump = true;
        let is_stable = false;

        assert_eq!(is_stable, jump_direction_is_stable(&original_jump, &current_jump));
    }

    #[test]
    fn jump_is_stable() {
        let original_jump = true;
        let current_jump = true;
        let is_stable = true;

        assert_eq!(is_stable, jump_direction_is_stable(&original_jump, &current_jump));
    }

    
    #[test]
    fn parses_line() {
        let line = "41 44 45 48 49 50 50";
        let parsed_line: Vec<i16> = vec![41, 44, 45, 48, 49, 50, 50];

        assert_eq!(parsed_line, parse_line(&line));
    }

    #[test]
    fn line_is_safe() {
        let line: Vec<i16> = vec![41, 44, 45, 48, 49, 50];
        let is_safe = true;

        assert_eq!(is_safe, is_safe_line(&line));
    }


    #[test]
    fn line_is_safe_basic() {
        let line: Vec<i16> = vec![41, 42];
        let is_safe = true;

        assert_eq!(is_safe, is_safe_line(&line));
    }

    #[test]
    fn line_is_unsafe_unstable_direction() {
        let line: Vec<i16> = vec![41, 44, 43, 48, 49, 50];
        let is_safe = false;

        assert_eq!(is_safe, is_safe_line(&line));
    }

    #[test]
    fn line_is_unsafe_jump_size() {
        let line: Vec<i16> = vec![41, 44, 45, 48, 52, 53];
        let is_safe = false;

        assert_eq!(is_safe, is_safe_line(&line));
    }
}
