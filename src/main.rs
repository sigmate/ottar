// Diamond (carreau)
// Club (trèfle)
// Spade (pique)
// Heart (coeur)
// Trump (atout)

// Unicode characters:
// Fool: ★
//

// Notes:
// Thoughts on a (Partial)Ord implementation for trick winner checking:
// When comparing two cards, the first one is always assumed to have the
// asked color (by the first player). So when comparing to a card from another
// suit, the first one will win. Inside a whole trick, it should be sufficient
// to keep track of the leading card. The rest of the (Partial)Ord implementation
// should handle the "natural" rules (e.g. Trump over Suit).
//
// Points for each Card is stored as u8 and is considered multiplied by 10.
// This is because Rust doesn't provide the Hash derive macro for floats (for understandable reasons).
//
// What's needed next:
// A common deck of card (and a method to generate one)
// A Player struct
// Some arbiter

extern crate strum;
#[macro_use] extern crate strum_macros;

use console::style;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use strum::IntoEnumIterator;

/*
struct Deck {

}
*/

#[derive(EnumIter, Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Spade => write!(f, "♠"),
            Suit::Heart => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Club => write!(f, "♣"),
        }
    }
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

#[derive(EnumIter, Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug)]
enum Rank {
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
    Knight,
    Queen,
    King,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Ace => write!(f, " 1"),
            Rank::Two => write!(f, " 2"),
            Rank::Three => write!(f, " 3"),
            Rank::Four => write!(f, " 4"),
            Rank::Five => write!(f, " 5"),
            Rank::Six => write!(f, " 6"),
            Rank::Seven => write!(f, " 7"),
            Rank::Eight => write!(f, " 8"),
            Rank::Nine => write!(f, " 9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, " V"),
            Rank::Knight => write!(f, " C"),
            Rank::Queen => write!(f, " D"),
            Rank::King => write!(f, " R"),
        }
    }
}

#[derive(EnumIter, Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug)]
enum Trump {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
}

impl fmt::Display for Trump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Trump::One => write!(f, " 1"),
            Trump::Two => write!(f, " 2"),
            Trump::Three => write!(f, " 3"),
            Trump::Four => write!(f, " 4"),
            Trump::Five => write!(f, " 5"),
            Trump::Six => write!(f, " 6"),
            Trump::Seven => write!(f, " 7"),
            Trump::Eight => write!(f, " 8"),
            Trump::Nine => write!(f, " 9"),
            Trump::Ten => write!(f, "10"),
            Trump::Eleven => write!(f, "11"),
            Trump::Twelve => write!(f, "12"),
            Trump::Thirteen => write!(f, "13"),
            Trump::Fourteen => write!(f, "14"),
            Trump::Fifteen => write!(f, "15"),
            Trump::Sixteen => write!(f, "16"),
            Trump::Seventeen => write!(f, "17"),
            Trump::Eighteen => write!(f, "18"),
            Trump::Nineteen => write!(f, "19"),
            Trump::Twenty => write!(f, "20"),
            Trump::TwentyOne => write!(f, "21"),
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Hash, Debug)]
enum Figure {
    Fool,
    Base(Suit, Rank),
    Trump(Trump),
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Figure::Fool => write!(f, "{}", style(/*"★ ★"*/ "* *").black().on_white()),
            Figure::Base(suit, rank) => match suit {
                Suit::Spade => write!(
                    f,
                    "{}",
                    style(format!("{}{}", suit, rank)).black().on_white()
                ),
                Suit::Heart => write!(
                    f,
                    "{}",
                    style(format!("{}{}", suit, rank)).red().on_white()
                ),
                Suit::Diamond => write!(
                    f,
                    "{}",
                    style(format!("{}{}", suit, rank)).red().on_white()
                ),
                Suit::Club => write!(
                    f,
                    "{}",
                    style(format!("{}{}", suit, rank)).black().on_white()
                ),
            },
            Figure::Trump(trump) => {
                write!(f, "{}", style(format!("⸬{}", trump)).black().on_white())
            }
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Hash, Debug)]
struct Card(Figure);

impl Card {
    fn points(&self) -> u8 {
        match self {
            Card(Figure::Fool) => 45,
            Card(Figure::Trump(Trump::One)) => 45,
            Card(Figure::Trump(Trump::TwentyOne)) => 45,
            Card(Figure::Base(_suit, rank)) => match rank {
                Rank::Jack => 15,
                Rank::Knight => 25,
                Rank::Queen => 35,
                Rank::King => 45,
                _ => 5
            },
            _ => 5
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Deck {
    set: HashSet<Card>,
}

impl Deck {
    fn new() -> Self {
        let mut deck = Self {
            set: HashSet::new()
        };
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                &deck.set.insert(Card(Figure::Base(suit, rank)));
            }
            
        }
        for trump in Trump::iter() {
            &deck.set.insert(Card(Figure::Trump(trump)));
        }
        &deck.set.insert(Card(Figure::Fool));
        deck
    }
    fn points(&self) -> u16 {
        let mut res: u16 = 0;
        for card in self.set.iter() {
            res += card.points() as u16;
        }
        res
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.set.iter().fold(String::new(), |mut acc, card| { acc.push_str(&card.to_string()); acc.push_str(" "); acc }))
    }
}

fn main() {

    let mut deck = Deck::new();
    println!("{}", deck);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_equality() {
        let card_a = Card(Figure::Base(Suit::Spade, Rank::Ace));
        let card_b = Card(Figure::Base(Suit::Spade, Rank::Ace));
        assert_eq!(card_a, card_b);
    }

    #[test]
    fn test_card_nonequality() {
        let card_a = Card(Figure::Base(Suit::Spade, Rank::Ace));
        let card_b = Card(Figure::Base(Suit::Spade, Rank::Two));
        let card_c = Card(Figure::Base(Suit::Diamond, Rank::Ace));
        let card_d = Card(Figure::Trump(Trump::One));
        let card_e = Card(Figure::Trump(Trump::TwentyOne));
        let card_f = Card(Figure::Fool);
        assert_ne!(card_a, card_b);
        assert_ne!(card_a, card_c);
        assert_ne!(card_a, card_d);
        assert_ne!(card_a, card_f);
        assert_ne!(card_d, card_e);
        assert_ne!(card_d, card_f);
    }

    #[test]
    fn test_card_comparison() {
        let card_a = Card(Figure::Base(Suit::Spade, Rank::Ace));
        let card_b = Card(Figure::Base(Suit::Spade, Rank::Two));
        let card_c = Card(Figure::Base(Suit::Diamond, Rank::Ace));
        let card_d = Card(Figure::Trump(Trump::One));
        let card_e = Card(Figure::Trump(Trump::TwentyOne));
        let card_f = Card(Figure::Fool);
        assert!(card_b > card_a);
        assert!(card_a > card_c);
        assert!(card_d > card_a);
        assert!(card_e > card_d);
        assert!(card_f < card_a);
    }

    #[test]
    fn test_card_unicity_in_hashmap() {
        let mut cards = HashSet::new();
        cards.insert(Card(Figure::Base(Suit::Spade, Rank::Ace)));
        cards.insert(Card(Figure::Trump(Trump::One)));
        cards.insert(Card(Figure::Fool));
        // voluntarily trying to add an already existing card
        cards.insert(Card(Figure::Base(Suit::Spade, Rank::Ace)));
        assert!(cards.len() == 3);
    }
}
