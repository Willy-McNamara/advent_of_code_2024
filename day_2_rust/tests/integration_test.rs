use std::fs;
use day_2_rust::run_parse_and_test;

#[test]
fn integration_one_result() {
    let file = "24 25 28 31 28
41 44 45 48 49 50 50
5 8 10 13 15 16 17 20
11 13 16 17 19 26
79 81 78 79 82 84";

    let valid_result_count = run_parse_and_test(&file);

    assert_eq!(valid_result_count, 1);
}


#[test]
fn integration_two_results() {
    let file = "24 25 28 31 28
41 44 45 48 49 50 50
5 8 10 13 15 16 17 20
11 13 16 17 19 21
79 81 78 79 82 84";
    let valid_result_count = run_parse_and_test(&file);

    assert_eq!(valid_result_count, 2);
}


#[test]
fn integration_no_results() {
    let file = "24 25 28 31 28
41 44 45 48 49 50 50
5 8 10 13 15 16 17 21
11 13 16 17 19 26
79 81 78 79 82 84";
    let valid_result_count = run_parse_and_test(&file);

    assert_eq!(valid_result_count, 0);
}
