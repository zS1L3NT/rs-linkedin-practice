use std::fmt::Display;

#[allow(dead_code)]
fn info<T: Display>(_: &T) {
    
}

#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}
