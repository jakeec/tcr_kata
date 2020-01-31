#![allow(dead_code)]
#![allow(unused_variables)]

use std::isize;

pub fn add(numbers: &str) -> Result<isize, String> {
    let numbers: Vec<isize> = numbers
        .split_whitespace()
        .map(|item| item.parse::<isize>().unwrap())
        .collect();

    let mut sum = 0;
    for number in numbers {
        if number < 0 {
            return Err(String::from("Number may not be negative!"));
        }
        if number <= 1000 {
            sum += number;
        }
    }

    Ok(sum)
}

#[cfg(test)]
pub mod add {
    use super::*;

    #[test]
    fn given_empty_string_return_0() {
        let input = "0";
        let result = add(input);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn given_list_of_numbers_return_sum() {
        let input = "1 2";
        let result = add(input);
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn given_newline_treat_it_as_whitespace() {
        let input = "1 2 \n5";
        let result = add(input);
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn given_negative_number_return_err() {
        let input = "1 2 \n-5";
        let result = add(input);
        match result {
            Err(error) => assert_eq!(error, "Number may not be negative!"),
            _ => panic!("Should throw error!"),
        }
    }

    #[test]
    fn given_number_greater_than_1000_ignore_it() {
        let input = "1 1001";
        let result = add(input);
        assert_eq!(result.unwrap(), 1);
    }
}
