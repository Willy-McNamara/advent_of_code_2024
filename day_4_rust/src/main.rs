use day_4_rust::read_matrix_from_file;
use day_4_rust::iter_and_count_successes;

fn main() {
    let filename = "input.txt";

    let matrix = read_matrix_from_file(&filename)
        .expect("error reading file to matrix");

    let total = iter_and_count_successes(&matrix);

    println!("The total XMAS count is {}", total);
}
