use std::{env, fs};
use std::time::Instant;
use crate::HandType::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};

fn main() {
    println!("Day 07 | Rust");
    let path = env::args().nth(1).expect("Missing required argument!");
    let input = fs::read_to_string(path).expect("Unable to read file!");

    let now = Instant::now();

    // parse hands and bids
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() { continue }
        let split: Vec<&str> = line.split_whitespace().collect();
        let hand = split[0];
        let bid: usize = split[1].parse().unwrap();

        let mut cards: Vec<Card> = Vec::new();
        for card in hand.chars() {
            cards.push(Card::from(card))
        }
        hands.push(Hand {cards, bid})
    }

    // sort hands
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..hands.len() - 1 {
            let current = &hands[i];
            let next = &hands[i + 1];
            if compare_hand(&current.cards, &next.cards) {
                sorted = false;
                hands.swap(i, i + 1)
            }
        }
    }

    // println!("Hands Sorted:");
    // for i in 0..hands.len() {
    //     print!("Hand {i}: ");
    //     for c in &hands[i].cards {
    //         print!("{:?} ", c)
    //     }
    //     println!("{}", &hands[i].bid)
    // }

    // calculate winnings and sum
    let mut total_winnings = 0;
    for i in 0..hands.len() {
        let rank = i + 1;
        let bid = &hands[i].bid;
        let winnings = rank * bid;
        total_winnings += winnings;

        // println!("Game {i}: rank {rank} bid {bid}")
    }

    let part1_ms = now.elapsed().as_millis();

    println!("Total winnings: {total_winnings}");
    println!("Took {part1_ms}ms");
}

// A set of 5 cards and a bid
struct Hand {
    cards: Vec<Card>,
    bid: usize
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack ,
    Queen ,
    King ,
    Ace
}
impl Card {
    fn value(&self) -> usize {
        match *self {
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::Ten => 9,
            Card::Jack => 10,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13
        }
    }
    fn from(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("That's not a card!")
        }
    }
}
#[derive(PartialEq, Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard
}
impl HandType {
    fn value(&self) -> usize {
        match *self {
            FiveOfKind => 7,
            FourOfKind => 6,
            FullHouse => 5,
            ThreeOfKind => 4,
            TwoPair => 3,
            OnePair => 2,
            HighCard => 1
        }
    }
}

// true if first is better, false if second
fn compare_hand(hand1: &Vec<Card>, hand2: &Vec<Card>) -> bool {
    let hand1_type = get_hand_type(hand1);
    let hand2_type = get_hand_type(hand2);
    if hand1_type != hand2_type {
        return hand1_type.value() > hand2_type.value()
    } else {
        for i in 0..5 {
            let hand1_card = &hand1[i];
            let hand2_card = &hand2[i];
            if hand1_card == hand2_card { continue }
            return hand1_card.value() > hand2_card.value()
        }
    }

    unreachable!()
}

fn get_hand_type(hand: &Vec<Card>) -> HandType {
    // count same cards
    let mut most_same: usize = 0;
    for i in 0..hand.len() {
        let mut count = 0;
        let card = &hand[i];
        for c in hand {
            if card == c {
                count += 1;
            }
        }
        if count > most_same {
            most_same = count;
        }
    }

    // count different cards
    let mut uniques: Vec<Card> = Vec::new();
    for card in hand {
        if !uniques.contains(card) {
            uniques.push(card.clone())
        }
    }
    return match uniques.len() {
        1 => FiveOfKind,
        2 => if most_same == 4 { FourOfKind } else { FullHouse },
        3 => if most_same == 3 { ThreeOfKind } else { TwoPair },
        4 => OnePair,
        5 => HighCard,
        _ => unreachable!()
    };
}