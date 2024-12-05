use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_matrix_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut matrix = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    Ok(matrix)
}

pub fn XMAS_checker(index: usize, letter: char) -> Result<(), String> {
    let actual = "XMAS";
    if actual.chars().nth(index) == Some(letter) {
        Ok(())
    } else {
        Err(format!("Well that there ain't a proper XMAS!"))
    }
}

pub fn traverser(coords: &Coordinates, step_dir: StepDirection, matrix: &Vec<Vec<char>>) -> Result<bool, String> {
    let mut step_count = 1;
    
    while step_count < 4 {
        // check all iterations
        match step_dir {
            StepDirection::Forward => {
                let new_col = coords.col + step_count;
                XMAS_checker(step_count, matrix[coords.row][new_col])?;
            }
            
            StepDirection::Backward => {
                let new_col = coords.col - step_count;
                XMAS_checker(step_count, matrix[coords.row][new_col])?;
            }

            StepDirection::Up => {
                let new_row = coords.row - step_count;
                XMAS_checker(step_count, matrix[new_row][coords.col])?;
            }
            StepDirection::Down => {
                let new_row = coords.row + step_count;
                XMAS_checker(step_count, matrix[new_row][coords.col])?;
            }
            StepDirection::ForUpDiag => {
                let new_row = coords.row - step_count;
                let new_col = coords.col + step_count;
                XMAS_checker(step_count, matrix[new_row][new_col])?;
            }
            StepDirection::ForDownDiag => {
                let new_row = coords.row + step_count;
                let new_col = coords.col + step_count;
                XMAS_checker(step_count, matrix[new_row][new_col])?;
            }
            StepDirection::BackUpDiag => {
                let new_row = coords.row - step_count;
                let new_col = coords.col - step_count;
                XMAS_checker(step_count, matrix[new_row][new_col])?;
            }
            StepDirection::BackDownDiag => {
                let new_row = coords.row + step_count;
                let new_col = coords.col - step_count;
                XMAS_checker(step_count, matrix[new_row][new_col])?;
            }
        }
        step_count += 1;
    }
    Ok(true)
}

pub fn run_traversal(coords: &Coordinates, matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    let can_up = coords.row > 2;
    let can_down = coords.row < (matrix.len() - 3);
    let can_back = coords.col > 2;
    let can_for = coords.col < (matrix[0].len() - 3);

    if can_up == true {
        if can_back == true {
            if let Ok(_) = traverser(&coords, StepDirection::BackUpDiag, matrix) {
                count += 1;
            }
        }
        if can_for == true {
            if let Ok(_) = traverser(&coords, StepDirection::ForUpDiag, matrix) {
                count += 1;
            }
        }
        if let Ok(_) = traverser(&coords, StepDirection::Up, matrix) {
            count += 1;
        }
    }
    if can_down == true {
        if can_back == true {
            if let Ok(_) = traverser(&coords, StepDirection::BackDownDiag, matrix) {
                count += 1;
            }
        } 
        if can_for == true {
            if let Ok(_) = traverser(&coords, StepDirection::ForDownDiag, matrix) {
                count += 1;
            }
        }
        if let Ok(_) = traverser(&coords, StepDirection::Down, matrix) {
            count += 1;
        }
    }
    if can_for == true {
        if let Ok(_) = traverser(&coords, StepDirection::Forward, matrix) {
            count += 1;
        }
    }
    if can_back == true {
        if let Ok(_) = traverser(&coords, StepDirection::Backward, matrix) {
            count += 1;
        }
    }
    
    count
}

pub fn iter_and_count_successes(matrix: &Vec<Vec<char>>) -> i32 {
    let mut success_count = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char == 'X' {
                let coords = Coordinates {
                    row: i,
                    col: j
                };
                let successes = run_traversal(&coords, matrix);
                success_count += successes;
            }
        }
    }

    success_count
}

pub struct Coordinates {
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub enum StepDirection {
    Forward,
    Backward,
    Up,
    Down,
    ForUpDiag,
    ForDownDiag,
    BackUpDiag,
    BackDownDiag,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn valid_letter_checker() {
        let valid_XMAS = "XMAS";
        let valid_index = 2;
        let valid_letter = 'A';

        assert_eq!(Ok(()), XMAS_checker(valid_index, valid_letter));
    }

    #[test]
    fn traverses_hori() {
        let test_coords = Coordinates {
            row: 0,
            col: 5,
        };
        let test_dir = StepDirection::Forward;
        let matrix = creates_dummy_matrix();
        
        assert_eq!(Ok(true), traverser(&test_coords, test_dir, &matrix));
    }


    #[test]
    fn traverses_verti_up() {
        let test_coords = Coordinates {
            row: 4,
            col: 6,
        };
        let test_dir = StepDirection::Up;
        let matrix = creates_dummy_matrix();
        
        assert_eq!(Ok(true), traverser(&test_coords, test_dir, &matrix));
    }

    #[test]
    fn traverses_diag_back_down() {
        let test_coords = Coordinates {
            row: 3,
            col: 9,
        }; 
        let test_dir = StepDirection::BackDownDiag;
        let matrix = creates_dummy_matrix();
        
        assert_eq!(Ok(true), traverser(&test_coords, test_dir, &matrix));
    }

    #[test]
    fn traverses_all_directions() {
        let test_coords = Coordinates {
            row: 9,
            col: 3,
        };
        let successes = 2;
        let matrix = creates_dummy_matrix();
        
        assert_eq!(successes, run_traversal(&test_coords, &matrix));
    }

    #[test]
    fn creates_matrix() {
        let dummy_input = "src/dummy_input.txt";
        let matrix = creates_dummy_matrix();

        assert_eq!(matrix, read_matrix_from_file(&dummy_input).unwrap());
    }

    fn creates_dummy_matrix() -> Vec<Vec<char>> {
        let dummy_data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        
        let matrix: Vec<Vec<char>> = dummy_data.lines()
            .map(|line| line.chars().collect())
            .collect();

        matrix
    }

    #[test]
    fn int_test_one() {
        let successes = 18;
        let test_matrix = creates_dummy_matrix();

        assert_eq!(successes, iter_and_count_successes(&test_matrix));
    }
}
