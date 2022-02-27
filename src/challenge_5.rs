use std::fmt::Display;

use Pulse::*;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

type Message = Vec<Letter>;

type Letter = Vec<Pulse>;

#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Short => write!(f, "."),
            Long => write!(f, "-"),
        }
    }
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut message: Message = Vec::with_capacity(self.len());

        for letter in self.chars() {
            if letter.is_alphabetic() {
                message.push(match letter {
                    'a' | 'A' => vec![Short, Long],
                    'b' | 'B' => vec![Long, Short, Short, Short],
                    'c' | 'C' => vec![Long, Short, Long, Short],
                    'd' | 'D' => vec![Long, Short, Short],
                    'e' | 'E' => vec![Short],
                    'f' | 'F' => vec![Short, Short, Long, Short],
                    'g' | 'G' => vec![Long, Long, Short],
                    'h' | 'H' => vec![Short, Short, Short, Short],
                    'i' | 'I' => vec![Short, Short],
                    'j' | 'J' => vec![Short, Long, Long, Long],
                    'k' | 'K' => vec![Long, Short, Long],
                    'l' | 'L' => vec![Short, Long, Short, Short],
                    'm' | 'M' => vec![Long, Long],
                    'n' | 'N' => vec![Long, Short],
                    'o' | 'O' => vec![Long, Long, Long],
                    'p' | 'P' => vec![Short, Long, Long, Short],
                    'q' | 'Q' => vec![Long, Long, Short, Long],
                    'r' | 'R' => vec![Short, Long, Short],
                    's' | 'S' => vec![Short, Short, Short],
                    't' | 'T' => vec![Long],
                    'u' | 'U' => vec![Short, Short, Long],
                    'v' | 'V' => vec![Short, Short, Short, Long],
                    'w' | 'W' => vec![Short, Long, Long],
                    'x' | 'X' => vec![Long, Short, Short, Long],
                    'y' | 'Y' => vec![Long, Short, Long, Long],
                    'z' | 'Z' => vec![Long, Long, Short, Short],
                    '0' => vec![Long, Long, Long, Long, Long],
                    '1' => vec![Short, Long, Long, Long, Long],
                    '2' => vec![Short, Short, Long, Long, Long],
                    '3' => vec![Short, Short, Short, Long, Long],
                    '4' => vec![Short, Short, Short, Short, Long],
                    '5' => vec![Short, Short, Short, Short, Short],
                    '6' => vec![Long, Short, Short, Short, Short],
                    '7' => vec![Long, Long, Short, Short, Short],
                    '8' => vec![Long, Long, Long, Short, Short],
                    '9' => vec![Long, Long, Long, Long, Short],
                    _ => continue,
                })
            }
        }

        message
    }
}

#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}
