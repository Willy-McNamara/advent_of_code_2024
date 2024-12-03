use std::io;

pub fn tests_for_mul(mul_slice: &str) -> bool {
    true
}

pub fn factors_from_sequence(sequence: &str) -> io::Result<Factors> {
    let mut curr_num = String::new();
    let mut factors_vec = Vec::new();

    for (i, char) in sequence.chars().enumerate() {
        match char {
            '0'..='9' => {
                if curr_num.len() > 2 {
                    return Err(io::Error::new(io::ErrorKind::Other, "Invalid char: num too big"));
                } else {
                    curr_num.push(char);
                }
            }
            ',' => {
                let parsed_num = curr_num.parse().map_err(|_| io::Error::new(io::ErrorKind::Other, "cant turn string to num"))?;
                if factors_vec.len() > 0 {
                    return Err(io::Error::new(io::ErrorKind::Other, "Invalid char: second comma"));
                } else {
                    factors_vec.push(parsed_num);
                    curr_num.clear();
                }
            }
            ')' => {
                let parsed_num = curr_num.parse().map_err(|_| io::Error::new(io::ErrorKind::Other, "cant turn string to num"))?;
                if factors_vec.len() != 1 {
                    return Err(io::Error::new(io::ErrorKind::Other, "not enough nums to end Mut statement"));
                } else {
                    factors_vec.push(parsed_num);
                    break;
                }
            }
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Invalid char")),
        }
    }

    let factors_as_nums = Factors {
        one: factors_vec[0],
        two: factors_vec[1],
    };

    Ok(factors_as_nums)
}

pub fn run_parse(file: &str) -> i32 {
    let mut sum = 0;

    for (i, char) in file.chars().enumerate() {
        if char == 'm' {
            if tests_for_mul(&file[i..i+4]) {
                let factors: Factors = match factors_from_sequence(&file[i+4..i+12]) {
                    Ok(factors) => factors,
                    Err(_) => continue,
                };
                sum += factors.product();
            }
        }
    }

    sum
}

#[derive(PartialEq, Debug)]
pub struct Factors {
    one: i32,
    two: i32,
}

impl Factors {
    pub fn product(&self) -> i32 {
        self.one * self.two
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn finds_mul() {
        let test_slice = "mul(";

        assert_eq!(true, tests_for_mul(&test_slice));
    }

    #[test]
    fn gets_factors() {
        let test_sequence = "123,456)";
        let solution_factors = Factors {
            one: 123,
            two: 456,
        };

        assert_eq!(solution_factors, factors_from_sequence(test_sequence).unwrap());
    }

    #[test]
    fn rejects_invalid_sequence() {
        let test_sequence = "12,4567)";

        assert!(factors_from_sequence(test_sequence).is_err());
    }

    #[test]
    fn factor_multiplication() {
        let test_factors = Factors {
            one: 123,
            two: 456,
        };
        let solution_product = 56088;

        assert_eq!(solution_product, test_factors.product());
    }

    #[test]
    fn parses_complex_string() {
        let complex_str = "fdaed&^#%$mut(123,456))(()mu())mut()--mut(23,4(--mut(55,555)--*";
        let product_sum = 86613;
        
        assert_eq!(product_sum, run_parse(&complex_str));
    }

    #[test]
    fn parses_complex_string_2() {
        let complex_str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))1234567"; 
        let product_sum = 161;
        
        assert_eq!(product_sum, run_parse(&complex_str));
    }

}
