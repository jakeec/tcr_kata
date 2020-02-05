#[derive(Debug)]
enum StringAddError {
    ArgumentNotValid(String),
}

fn string_add(a: &str, b: &str) -> Result<String, StringAddError> {
    let a_chars = a.chars().rev();
    let mut b_chars = b.chars().rev();
    let mut output_string = String::from("");
    let mut carry = 0;

    for a_char in a_chars {
        let mut sum: u32 = carry
            + a_char.to_digit(10).unwrap()
            + b_chars.next().take().unwrap().to_digit(10).unwrap();
        if carry != 0 {
            carry = 0;
        }
        if sum >= 10 {
            carry = 1;
            sum = sum - 10;
        }
        output_string.push_str(&sum.to_string());
    }

    if carry > 0 {
        output_string.push_str("1");
    }

    output_string = output_string.chars().rev().collect();

    Ok(output_string)
}

fn hex_string_add(a: &str, b: &str) -> Result<String, StringAddError> {
    use std::u32;
    let a_int: u32 = a.parse().unwrap();
    let b_int: u32 = b.parse().unwrap();
    let a_chars = format!("{:X}", a_int);
    let a_chars = a_chars.chars().rev();
    let b_chars = format!("{:X}", b_int);
    let mut b_chars = b_chars.chars().rev();
    let mut output_string = String::from("");
    let mut carry = 0;

    println!("{:?}", a_chars);
    println!("{:?}", b_chars);

    for a_char in a_chars {
        let mut sum: u32 = carry
            + u32::from_str_radix(&a_char.to_string(), 16).unwrap()
            + u32::from_str_radix(&b_chars.next().take().unwrap().to_string(), 16).unwrap();
        println!("{}", sum);
        if carry != 0 {
            carry = 0;
        }
        if sum >= 16 {
            carry = 1;
            sum = sum - 16;
        }
        output_string.push_str(&format!("{:X}", &sum));
    }

    if carry > 0 {
        output_string.push_str("1");
    }

    output_string = output_string.chars().rev().collect();
    println!("{}", output_string);
    // let output_string = format!("{:X}", &u32::from_str_radix(&output_string, 10).unwrap());
    let output_string = u32::from_str_radix(&output_string, 16).unwrap().to_string();
    Ok(output_string)
}

#[cfg(test)]
mod string_adder_tests {
    use super::*;

    mod string_add_tests {
        use super::*;

        #[test]
        fn given_1_1_return_2() {
            let result = string_add("1", "1");
            assert_eq!(result.unwrap(), String::from("2"));
        }
        #[test]
        fn given_5_5_return_10() {
            let result = string_add("5", "5");
            assert_eq!(result.unwrap(), String::from("10"));
        }
        #[test]
        fn given_123456_123456_return_246912() {
            let result = string_add("123456", "123456");
            assert_eq!(result.unwrap(), String::from("246912"));
        }
        #[test]
        fn given_543210_543210_return_1086420() {
            let result = string_add("543210", "543210");
            assert_eq!(result.unwrap(), String::from("1086420"));
        }
    }

    mod hex_string_add_tests {
        use super::*;

        #[test]
        fn given_12_12_return_24() {
            let result = hex_string_add("12", "12");
            assert_eq!(result.unwrap(), String::from("24"));
        }

        #[test]
        fn given_67587_78122_return_145709() {
            let result = hex_string_add("67587", "78122");
            assert_eq!(result.unwrap(), String::from("145709"));
        }

        #[test]
        fn given_0x0F_0x0F_return_145709() {
            let result = hex_string_add("0x0F", "0x0F");
            assert_eq!(result.unwrap(), String::from("145709"));
        }
    }
}
