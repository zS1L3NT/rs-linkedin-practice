use std::str::FromStr;

#[allow(dead_code)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug, Clone)]
enum ParseError {
    TooShort,
    TooLong,
    InvalidChecksum,
    InvalidCharacter(usize, char),
}

impl FromStr for Isbn {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("-", "");
        for (i, c) in s.chars().enumerate() {
            if !c.is_digit(10) {
                return Err(ParseError::InvalidCharacter(i, c));
            }
        }

        if s.len() < 13 {
            return Err(ParseError::TooShort);
        }

        if s.len() > 13 {
            return Err(ParseError::TooLong);
        }

        let digits = &s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>()[..];

        if calculate_check_digit(&digits[..12]) != digits[12] {
            return Err(ParseError::InvalidChecksum);
        }

        Ok(Isbn {
            raw: s.to_string(),
            digits: digits.to_vec(),
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let sum = digits
        .iter()
        .enumerate()
        .map(|(i, d)| if i % 2 == 0 { *d } else { d * 3 })
        .sum::<u8>()
        % 10;
    (10 - sum) % 10
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let isbn: Isbn = "978-3-16-148410-0".parse().unwrap();
    assert_eq!(isbn.raw, "9783161484100");
    assert_eq!(isbn.digits, [9, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0, 0]);
}
