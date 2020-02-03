extern crate regex;

use regex::Regex;

pub type VerificationRules =
    Vec<fn(conditions_met: &mut usize) -> fn(password: &str) -> Result<(), VerificationError>>;

pub struct PasswordVerifier {
    verification_rules: VerificationRules,
}

pub enum VerificationError {
    NoLowerCaseLetterErr(String),
    MinimumCriteriaNotMetErr(String),
}

impl PasswordVerifier {
    pub fn new(verification_rules: VerificationRules) -> Self {
        Self {
            verification_rules: verification_rules,
        }
    }

    pub fn verify(self, password: &str) -> Result<String, VerificationError> {
        let upper_case_regex = Regex::new(r"[A-Z]").unwrap();
        let lower_case_regex = Regex::new(r"[a-z]").unwrap();
        let numeric_regex = Regex::new(r"[0-9]").unwrap();
        let mut conditions_met = 0;

        for rule in self.verification_rules {
            let rule_result = rule(&mut conditions_met)(&password);
            match rule_result {
                Err(err) => return Err(err),
                _ => (),
            }
        }

        if !lower_case_regex.is_match(password) {
            return Err(VerificationError::NoLowerCaseLetterErr(String::from(
                "Password must contain at least one lower case letter!",
            )));
        }
        if password.len() > 0 {
            conditions_met += 1;
        }
        if password.len() > 8 {
            conditions_met += 1;
        }
        if upper_case_regex.is_match(password) {
            conditions_met += 1;
        }
        if numeric_regex.is_match(password) {
            conditions_met += 1;
        }

        if conditions_met >= 3 {
            return Ok(String::from("Password is acceptable."));
        } else {
            return Err(VerificationError::MinimumCriteriaNotMetErr(String::from(
                "Password must meet at least 3 of the acceptance conditions!",
            )));
        }
    }
}

#[cfg(test)]
pub mod PasswordVerifierTests {
    use super::*;

    #[test]
    fn given_valid_password_return_ok() {
        let rules = vec![|conditions_met: &mut usize| {
            |password: &str| {
                let upper_case_regex = Regex::new(r"[A-Z]").unwrap();
                if upper_case_regex.is_match(password) {
                    *conditions_met += 1;
                }
                Ok(())
            }
        }];
        let verifier = PasswordVerifier::new(rules);
        let result = verifier.verify("aBcdEfghIjK123");
        match result {
            Ok(msg) => assert_eq!(msg, String::from("Password is acceptable.")),
            _ => panic!("Expected Ok!"),
        }
    }

    // #[test]
    // fn given_password_meets_3_criteria_return_ok() {
    //     let result = PasswordVerifier::verify("aBcdEfghIjK");
    //     match result {
    //         Ok(msg) => assert_eq!(msg, String::from("Password is acceptable.")),
    //         _ => panic!("Expected Ok!"),
    //     }
    // }

    // #[test]
    // fn given_password_meets_2_criteria_return_err() {
    //     let result = PasswordVerifier::verify("abcdefghijk");
    //     match result {
    //         Err(err) => match err {
    //             VerificationError::MinimumCriteriaNotMetErr(_) => (),
    //             _ => panic!("Expected MinimumCriteriaNotMetErr"),
    //         },
    //         _ => panic!("Expected Err!"),
    //     }
    // }

    // #[test]
    // fn given_password_meets_1_criterion_return_err() {
    //     let result = PasswordVerifier::verify("abc");
    //     match result {
    //         Err(err) => match err {
    //             VerificationError::MinimumCriteriaNotMetErr(_) => (),
    //             _ => panic!("Expected MinimumCriteriaNotMetErr"),
    //         },
    //         _ => panic!("Expected Err!"),
    //     }
    // }

    // #[test]
    // fn given_password_contains_no_lowercase_letters_return_err() {
    //     let result = PasswordVerifier::verify("ABCDEFGHIKL123");
    //     match result {
    //         Err(err) => match err {
    //             VerificationError::NoLowerCaseLetterErr(_) => (),
    //             _ => panic!("Expected NoLowerCaseLetterErr"),
    //         },
    //         _ => panic!("Expected Err!"),
    //     }
    // }
}
