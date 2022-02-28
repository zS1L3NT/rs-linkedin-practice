use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

#[derive(Debug, Clone)]
enum ParseError {
    InvalidCharacter(usize, char),
    InvalidLength,
}

impl FromStr for Rgb {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            return Err(ParseError::InvalidLength);
        }

        if !s.starts_with("#") {
            return Err(ParseError::InvalidCharacter(0, s.chars().next().unwrap()));
        }

        if !&s[1..].chars().all(|c| c.is_digit(16)) {
            return Err(ParseError::InvalidCharacter(1, s.chars().nth(1).unwrap()));
        }

        Ok(Rgb {
            r: u8::from_str_radix(&s[1..3], 16).unwrap(),
            g: u8::from_str_radix(&s[3..5], 16).unwrap(),
            b: u8::from_str_radix(&s[5..7], 16).unwrap(),
        })
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
fn too_short() {
    assert!("1234".parse::<Rgb>().is_err());
}

#[test]
fn not_a_hex_code() {
    assert!("?".parse::<Rgb>().is_err());
}

#[test]
fn invalid_literals() {
    assert!("?".parse::<Rgb>().is_err());
}

#[test]
fn no_leading_hash() {
    assert!("aabbcc".parse::<Rgb>().is_err());
}

#[test]
fn out_of_bounds() {
    assert!("00gg00".parse::<Rgb>().is_err());
}
