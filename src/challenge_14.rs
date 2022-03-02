#![allow(dead_code)]

mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut chars: Vec<(char, u32)> = vec![];

        for c in text.chars() {
            if chars.len() == 0 {
                chars.push((c, 1));
            } else {
                let last_char = chars.last_mut().unwrap();
                if last_char.0 == c && last_char.1 != 9 {
                    last_char.1 += 1;
                } else {
                    chars.push((c, 1));
                }
            }
        }

        chars
            .iter()
            .map(|(ch, co)| format!("{}{}", co, ch))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn decode(text: &str) -> String {
        text.chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chars| (chars[0].to_digit(10).unwrap(), chars[1].to_owned()))
            .map(|(co, ch)| ch.to_string().repeat(co as usize))
            .collect::<Vec<_>>()
            .join("")
    }
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
