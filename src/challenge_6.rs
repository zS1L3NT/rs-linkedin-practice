#![allow(dead_code)]

#[derive(PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Self { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut value = 0;
        for card in &self.cards {
            match card {
                Card::Ace => value += 11,
                Card::Two => value += 2,
                Card::Three => value += 3,
                Card::Four => value += 4,
                Card::Five => value += 5,
                Card::Six => value += 6,
                Card::Seven => value += 7,
                Card::Eight => value += 8,
                Card::Nine => value += 9,
                Card::Ten | Card::Jack | Card::Queen | Card::King => value += 10,
            }
        }

        let ace_ct = self.cards.iter().filter(|n| **n == Card::Ace).count();
        if ace_ct > 0 && value > 21 {
            value -= 10 * ace_ct;
        }

        value
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}
