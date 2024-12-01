use std::fs;
use Day_1::run_parse;

#[test]    
fn integration_one() {
	
    let file_path = "test_input_two.txt";
    let file = fs::read_to_string(file_path)
        .expect("should be able to read in file");    

    let result: i32 = run_parse(&file);	
    assert_eq!(result, 68301);
}
