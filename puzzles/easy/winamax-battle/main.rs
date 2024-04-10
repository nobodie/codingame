// Puzzle link : https://www.codingame.com/ide/puzzle/winamax-battle

use std::{io, convert::TryFrom, collections::VecDeque};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Card {
    Value(u8),
    Jack,
    Queen,
    King,
    Ace
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with('J') {
            Ok(Self::Jack)
        } else if value.starts_with('Q'){
            Ok(Self::Queen)
        } else if value.starts_with('K') {
            Ok(Self::King)
        } else if value.starts_with('A') {
            Ok(Self::Ace)
        } else if let Ok(number) = (&value[..value.len() - 1]).parse::<u8>() {
            Ok(Self::Value(number))
        } else {
            Err("Invalid input".to_string())
        }
    }
}


#[derive(Default)]
pub struct Deck {
    deck: VecDeque<Card>,
    discard: VecDeque<Card>
}

impl Deck {
    pub fn add_card(&mut self, card: Card) {
        self.deck.push_back(card);
    }
    pub fn discard(&mut self, count: usize) -> Result<(), String>{
        for _ in 0..count {
            self.discard.push_back(self.deck.pop_front().ok_or("Deck is empty".to_string())?);
        }
        Ok(())
    }

    pub fn pick_card(&mut self) -> Result<Card, String> {
        let card = self.deck.pop_front().ok_or("Deck is empty".to_string())?;
        self.discard.push_back(card.clone());
        Ok(card)
    }

    pub fn win_battle(&mut self, other : &mut Self, take_first: bool) {
        if take_first {
            self.deck.append(&mut self.discard);
            self.deck.append(&mut other.discard);
        } else {
            self.deck.append(&mut other.discard);
            self.deck.append(&mut self.discard);
        }
    }

}


fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of cards for player 1

    let mut deck1 = Deck::default();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let cardp_1 = input_line.trim().to_string(); // the n cards of player 1
        //deck1.push_back(Card::try_from(cardp_1.as_str()).unwrap());
        deck1.add_card(Card::try_from(cardp_1.as_str()).unwrap());
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let m = parse_input!(input_line, i32); // the number of cards for player 2

    let mut deck2 = Deck::default();
    for _ in 0..m as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let cardp_2 = input_line.trim().to_string(); // the m cards of player 2
        //deck2.push_back(Card::try_from(cardp_2.as_str()).unwrap());
        deck2.add_card(Card::try_from(cardp_2.as_str()).unwrap());
    }

    let mut round_count = 0;
    let mut winner = None;
    let mut stop = false;

    while !stop {
        let card1 = deck1.pick_card();
        let card2 = deck2.pick_card();

        if card1.is_err() {
            winner = Some(2);
            stop = true;
        } else if card2.is_err() {
            winner = Some(1);
            stop = true;
        } else {
            let card1 = card1.unwrap();
            let card2 = card2.unwrap();

            if card1 == card2 {
                if deck1.discard(3).is_err() || deck2.discard(3).is_err() {
                    stop = true;
                }
            } else {
                round_count += 1;

                if card1 > card2 {
                    deck1.win_battle(&mut deck2, true);
                } else {
                    deck2.win_battle(&mut deck1, false);
                }
            }
        }
    }

    match winner {
        Some(player) => println!("{player} {round_count}"),
        None => println!("PAT"),
    };
}

