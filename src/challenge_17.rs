#![allow(dead_code)]

trait InlineRotate {
    fn rotate_left_inline(&self, n: usize) -> Self;
}

impl<T: Clone> InlineRotate for Vec<T> {
    fn rotate_left_inline(&self, n: usize) -> Self {
        let mut result = self.to_vec();
        result.rotate_left(n);
        result
    }
}

mod vigenere {
    use std::collections::HashMap;

    use super::InlineRotate;

    fn get_cipher() -> HashMap<char, HashMap<char, char>> {
        let alphabets = vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];

        alphabets
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let x = (
                    *c,
                    alphabets
                        .rotate_left_inline(i)
                        .iter()
                        .enumerate()
                        .map(|(i, c)| (alphabets[i], *c))
                        .collect::<HashMap<_, _>>(),
                );
                x
            })
            .collect::<HashMap<_, HashMap<_, _>>>()
    }

    fn get_keystream(key: &str, len: usize) -> Vec<char> {
        let mut keystream = key.to_string();

        while keystream.len() < len {
            keystream.push_str(&key);
        }

        keystream.truncate(len);
        keystream.chars().collect()
    }

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let cipher = get_cipher();

        let plaintext = plaintext
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| *c.to_uppercase().collect::<Vec<char>>().first().unwrap())
            .collect::<Vec<char>>();
        let keystream = get_keystream(key, plaintext.len());
        let mut ciphertext = String::new();

        for (char, key_char) in plaintext.iter().zip(keystream.iter()) {
            if !char.is_alphabetic() {
                continue;
            }

            ciphertext.push(*cipher.get(key_char).unwrap().get(char).unwrap());
        }

        ciphertext
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let cipher = get_cipher();

        let ciphertext = ciphertext
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| *c.to_uppercase().collect::<Vec<char>>().first().unwrap())
            .collect::<Vec<char>>();
        let keystream = get_keystream(key, ciphertext.len());
        let mut plaintext = String::new();

        for (char, key_char) in ciphertext.iter().zip(keystream.iter()) {
            if !char.is_alphabetic() {
                continue;
            }

            plaintext.push(
                *cipher
                    .get(key_char)
                    .unwrap()
                    .iter()
                    .find_map(|(k, v)| if v == char { Some(k) } else { None })
                    .unwrap(),
            );
        }

        plaintext
    }
}

#[test]
fn encrypt() {
    let key = "WHYRUST";
    let plaintext = "TOEMPOWEREVERYONE";
    let ciphertext = vigenere::encrypt(&plaintext, key);

    assert_eq!(ciphertext, "PVCDJGPAYCMYJRKUC");
}

#[test]
fn decrypt() {
    let key = "WHYRUST";
    let ciphertext = "PVCDJGPAYCMYJRKUC";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    assert_eq!(plaintext, "TOEMPOWEREVERYONE");
}

#[test]
fn two_way() {
    let key = "LEEJIEUN";
    let plaintext = "ILOVEIU";

    assert_eq!(plaintext, vigenere::decrypt(&vigenere::encrypt(&plaintext, &key), &key));
}