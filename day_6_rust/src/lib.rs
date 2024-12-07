
pub fn generate_map(file: &str) -> Map {
    let mut grid = Vec::new();
    for line in file.lines() {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }
    Map {
        grid: grid
    }
}

pub fn run_job(file: &str) -> i32 {
    let mut map = generate_map(file);
    let start = map.finds_starting_position().unwrap();
    map.walk(start.coords, start.dir);
    map.distinct_positions()
}



#[derive(PartialEq, Debug)]
pub struct Map {
    grid: Vec<Vec<char>>,
}

impl Map {
    
    fn finds_starting_position(&self) -> Option<Position> {

        for (i, row) in self.grid.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if let Some(position) = match char {
                    &'^' => Some(Position { dir: Direction::Up, coords: Coordinates { row: i, col: j }}),
                    &'>' => Some(Position { dir: Direction::Right, coords: Coordinates { row: i, col: j }}),
                    &'<' => Some(Position { dir: Direction::Left, coords: Coordinates { row: i, col: j }}),
                    &'v' => Some(Position { dir: Direction::Down, coords: Coordinates { row: i, col: j }}),
                    _ => continue,
                } {
                    return Some(position);
                }
            }
        }
        None
    }

    fn peak(&self, coords: &Coordinates, dir: &Direction) -> Steps {
        if self.checks_exit(&coords, &dir) {
            return Steps::End;
        };
        
        let next_coords = coords.next(dir);
        let next_char = &self.grid[next_coords.row][next_coords.col];
        if next_char == &'#' {
            return Steps::Obstacle;
        };

        Steps::Clear
    }

    fn checks_exit(&self, coords: &Coordinates, dir: &Direction) -> bool {
        match dir {
            Direction::Up => {
                coords.row == 0
            },
            Direction::Down => {
                coords.row > self.grid.len() - 1
            },
            Direction::Right => {
                coords.col > self.grid[0].len() -1
            },
            Direction::Left => {
                coords.col == 0
            },
        }
    }

    fn take_step(&mut self, coords: Coordinates) {
        self.grid[coords.row][coords.col] = 'X';
    }

    fn walk(&mut self, coords: Coordinates, dir: Direction) {
        match self.peak(&coords, &dir) {
            Steps::End => (),
            Steps::Obstacle => {
                dir.turn();
                self.walk(coords, dir);
            },
            Steps::Clear => {
                let next_coords = coords.next(&dir);
                self.take_step(coords);
                self.walk(next_coords, dir);
            }
        };
    }
    
    fn distinct_positions(&self) -> i32 {
        let mut counter = 0;
        for row in self.grid.iter() {
            for char in row.iter() {
                if char == &'X' {
                    counter += 1;
                };
            };
        };
        counter
    }
}


#[derive(PartialEq, Debug)]
pub struct Coordinates {
    row: usize,
    col: usize,
}

impl Coordinates {
    fn next(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Coordinates {
                row: self.row.saturating_sub(1),
                col: self.col,
            },
            Direction::Down => Coordinates {
                row: self.row + 1, 
                col: self.col,
            },
            Direction::Right => Coordinates {
                row: self.row,
                col: self.col + 1,
            },
            Direction::Left => Coordinates {
                row: self.row,
                col: self.col.saturating_sub(1),
            },
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}


#[derive(PartialEq, Debug)]
pub struct Position {
    coords: Coordinates,
    dir: Direction,
}

#[derive(PartialEq, Debug)]
pub enum Steps {
    End,
    Obstacle,
    Clear,
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_file_sm: &str = "....#.....
.........#";

    const test_file_start: &str = "..........
.#......v.";

    #[test]
    fn creates_map() {
        let mut test_row = vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'];
        let mut test_row_2 = vec![ '.', '.', '.', '.', '.',  '.', '.', '.', '.', '#'];
        let mut test_grid = vec![test_row, test_row_2];
        let mut test_map = Map {
            grid: test_grid,
        };

        assert_eq!(test_map, generate_map(test_file_sm));
    }

    #[test]
    fn returns_starting_coords() {
        let starting_coords = Coordinates {
            row: 1,
            col: 8,
        };
        let starting_dir = Direction::Down;
        let starting_position = Position {
            coords: starting_coords,
            dir: starting_dir,
        };

        assert_eq!(starting_position, generate_map(test_file_start).finds_starting_position().unwrap());
    }

    #[test]
    fn returns_next_coords() {
        let test_coords: Coordinates = Coordinates {
            row: 1,
            col: 1,
        };
        let test_dir = Direction::Left;
        let valid_next: Coordinates = Coordinates {
            row: 1,
            col: 0,
        };

        assert_eq!(valid_next, test_coords.next(&test_dir));
    }

    #[test]
    fn peaks_end() {
        let test_map = generate_map(test_file_start);
        let test_coords = Coordinates {
            row: 1, 
            col: 0,
        };
        let test_dir = Direction::Left;
        let solution_step = Steps::End;

        assert_eq!(solution_step, test_map.peak(&test_coords, &test_dir));
    }

    #[test]
    fn peaks_obstacle() {
        let test_map = generate_map(test_file_start);
        let test_coords = Coordinates {
            row: 0, 
            col: 1,
        };
        let test_dir = Direction::Down;
        let solution_step = Steps::Obstacle;

        assert_eq!(solution_step, test_map.peak(&test_coords, &test_dir));
    }

    #[test]
    fn peaks_clear() {
        let test_map = generate_map(test_file_start);
        let test_coords = Coordinates {
            row: 0, 
            col: 2,
        };
        let test_dir = Direction::Right;
        let solution_step = Steps::Clear;

        assert_eq!(solution_step, test_map.peak(&test_coords, &test_dir));
    }
}

