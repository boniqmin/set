use std::fmt::{Display, Error, Formatter};
// use yew::prelude::*;
use rand::seq::SliceRandom;

pub struct CardSelection {
    card_nums: Vec<Option<usize>>,
}

#[derive(Debug, Clone)]
pub struct FullError;

impl CardSelection {
    pub fn new() -> CardSelection {
        CardSelection {
            card_nums: vec![None, None, None],
        }
    }

    pub fn is_full(&self) -> bool {
        for card in self.card_nums.iter() {
            if card.is_none() {
                return false;
            }
        }
        true
    }

    // pub fn add_next(&mut self, next: usize) -> Result<(), FullError> {
    //     // returns
    //     for card in self.card_nums.iter_mut() {
    //         if card.is_none() {
    //             *card = Some(next);
    //             return Ok(());
    //         }
    //     }
    //     Err(FullError)
    // }

    pub fn add_next_toggle(&mut self, next: usize) -> Result<(), FullError> {
        // given index of card "next", this method either removes that card from the
        // selection if present, otherwise adds it to the selection.
        // returns FullError if next is not in selection and selection is full
        if self.is_selected(next) {
            for card in self.card_nums.iter_mut() {
                // remove all instances of next
                if card == &Some(next) {
                    *card = None;
                }
            }
            return Ok(()); // return here to not re-add
        }
        for card in self.card_nums.iter_mut() {
            if card.is_none() {
                *card = Some(next);
                return Ok(());
            }
        }
        Err(FullError)
    }

    pub fn is_set(&self, cards: &Vec<Card>) -> bool {
        if !self.is_full() {
            return false;
        }
        return is_set(
            &cards[self.card_nums[0].unwrap()],
            &cards[self.card_nums[1].unwrap()],
            &cards[self.card_nums[2].unwrap()],
        );
    }

    // pub fn replace_cards_random(&self, cards: &mut Vec<Card>) {
    //     for card_num in &self.card_nums {
    //         if let Some(i) = card_num {
    //             cards[*i] = Card::new_random();
    //         }
    //     }
    // }

    pub fn replace_cards_from_deck(&self, cards: &mut Vec<Card>, deck: &mut Deck) {
        // replaces the cards in "cards" at the indices of the selction by new ones from the deck
        let mut new_card_opts: Vec<Option<Card>> =
            cards.iter().map(|card| Some(card.clone())).collect();
        for card_num in &self.card_nums {
            if let Some(i) = card_num {
                let newcard = deck.draw();
                new_card_opts[*i] = newcard; //newcard.as_ref();
            }
        }

        // Filters out the None variants and unpacks the Some variants into cards
        *cards = new_card_opts
            .iter()
            .filter_map(|card| card.clone())
            .collect();

        // let new_cards = Vec::new();
        // for card_num in card_nums_sorted {
        //     if card_num.is_some() {
        //         match self.deck.draw() {
        //             Some(card) => new_cards.push(card)
        //         }

        //     }
        // }
    }

    // TODO: make this safe
    pub fn remove_cards(&self, cards: &mut Vec<Card>) {
        // this method removes the cards in the selection from the "cards" Vec, and fills
        let mut valid_card_nums = self
            .card_nums
            .iter()
            .filter_map(|num| num.as_ref())
            .collect::<Vec<&usize>>();
        valid_card_nums.sort(); // sort_by(|a, b| b.cmp(a));

        let mut cards_to_be_moved = Vec::new();
        for i in (cards.len() - 3)..cards.len() {
            if !valid_card_nums.contains(&&i) {
                cards_to_be_moved.push(cards[i].clone());
            }
        }
        log::info!("Cards in ctbm: {:?}", cards_to_be_moved);

        let new_cards = cards
            .clone()
            .iter()
            .enumerate()
            .filter_map(|(i, card)| {
                if i >= cards.len() - 3 {
                    None
                } else {
                    if valid_card_nums.contains(&&i) {
                        Some(cards_to_be_moved.pop().unwrap())
                    } else {
                        Some(card.clone())
                    }
                }
            })
            .collect::<Vec<Card>>();
        *cards = new_cards;
    }

    pub fn clear(&mut self) {
        self.card_nums = vec![None, None, None];
    }

    pub fn is_selected(&self, card_nr: usize) -> bool {
        self.card_nums.contains(&Some(card_nr))
    }
}

impl Display for CardSelection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut text = String::new();
        for opt_card in &self.card_nums {
            let card_repr = match opt_card {
                None => "-".to_owned(),
                Some(card) => {
                    let card = card.clone();
                    card.to_string()
                }
            };
            text.push_str(&card_repr);
        }
        write!(f, "{text} ")
    }
}

// fn randint(min: i64, max: i64) -> i64 {
//     let range = (max - min + 1) as f64;
//     (rand::random::<f64>() * range).floor() as i64 + min
// }

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    pub shape: u32,
    pub color: u32,
    pub filling: u32,
    pub amount: u32,
}

impl Card {
    // pub fn new_random() -> Card {
    //     Card {
    //         shape: randint(0, 2) as u32,
    //         color: randint(0, 2) as u32,
    //         filling: randint(0, 2) as u32,
    //         amount: randint(0, 2) as u32,
    //     }
    // }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}{}{}{}",
            self.color, self.shape, self.filling, self.amount
        )
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for shape in 0..3 {
            for color in 0..3 {
                for filling in 0..3 {
                    for amount in 0..3 {
                        cards.push(Card {
                            shape,
                            color,
                            filling,
                            amount,
                        })
                    }
                }
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn new_shuffled() -> Self {
        let mut new = Self::new();
        new.shuffle();
        new
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    // pub fn peek(&self) -> Option<&Card> {
    //     self.cards.last()
    // }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

// #[derive(Properties, PartialEq)]
// pub struct Board {
//     pub cards: Vec<Card>,
//     pub on_click: Callback<Card>,
// }

// impl Board {
//     // pub fn new_random(size: usize) -> Board {
//     //     let mut cards = Vec::new();
//     //     for _ in 0..size {
//     //         cards.push(Card::new_random())
//     //     }
//     //     Board { cards }
//     // }

//     pub fn replace_random(&mut self, idx: usize) {
//         self.cards[idx] = Card::new_random();
//     }
// }

fn all_same_or_different<T: PartialEq>(first: T, second: T, third: T) -> bool {
    if first == second && second == third {
        true
    } else if first != second && second != third && third != first {
        true
    } else {
        false
    }
}

pub fn is_set(first: &Card, second: &Card, third: &Card) -> bool {
    // true
    if !all_same_or_different(first.color, second.color, third.color) {
        false
    } else if !all_same_or_different(first.shape, second.shape, third.shape) {
        false
    } else if !all_same_or_different(first.filling, second.filling, third.filling) {
        false
    } else if !all_same_or_different(first.amount, second.amount, third.amount) {
        false
    } else {
        true
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_rng() {
//         for _ in 0..8 {
//             println! {"{}", Card::new_random()}
//         }
//     }
// }
