// Diamond (carreau)
// Club (trèfle)
// Spade (pique)
// Heart (coeur)
// Trump (atout)

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

use std::collections::HashSet;
use std::cmp::Ordering;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Debug)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self == other {
            true => Some(Ordering::Equal),
            false => Some(Ordering::Greater)
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Hash, Debug)]
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
    King
}

#[derive(Eq, PartialEq, PartialOrd, Hash, Debug)]
enum Trump {
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Eleven, Twelve, Thirteen, Fourteen, Fifteen, Sixteen, Seventeen, Eighteen, Nineteen, Twenty,
    TwentyOne
}

#[derive(Eq, PartialEq, PartialOrd, Hash, Debug)]
enum Figure {
    Fool,
    Base(Suit, Rank),
    Trump(Trump)
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Figure::Fool => write!(f, "Fool"),
            Figure::Base(suit, rank) => write!(f, "{:?} of {:?}s", rank, suit),
            Figure::Trump(trump) => write!(f, "{:?} of Trumps", trump),
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Hash, Debug)]
struct Card(Figure, u8);

fn main() {
    println!("Ottar");
    println!("{:?}", Card(Figure::Base(Suit::Spade, Rank::Ace), 5));
    println!("{}", Figure::Base(Suit::Spade, Rank::Ace));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_equality() {
        let card_a = Card(Figure::Base(Suit::Spade, Rank::Ace), 5);
        let card_b = Card(Figure::Base(Suit::Spade, Rank::Ace), 5);
        assert_eq!(card_a, card_b);
    }

    #[test]
    fn test_card_nonequality() {
        let card_a = Card(Figure::Base(Suit::Spade, Rank::Ace), 5);
        let card_b = Card(Figure::Base(Suit::Spade, Rank::Two), 5);
        let card_c = Card(Figure::Base(Suit::Diamond, Rank::Ace), 5);
        let card_d = Card(Figure::Trump(Trump::One), 45);
        let card_e = Card(Figure::Trump(Trump::TwentyOne), 45);
        let card_f = Card(Figure::Fool, 45);
        assert_ne!(card_a, card_b);
        assert_ne!(card_a, card_c);
        assert_ne!(card_a, card_d);
        assert_ne!(card_a, card_f);
        assert_ne!(card_d, card_e);
        assert_ne!(card_d, card_f);
    }

    #[test]
    fn test_card_comparison()
    {
        let card_a = Card(Figure::Base(Suit::Spade, Rank::Ace), 5);
        let card_b = Card(Figure::Base(Suit::Spade, Rank::Two), 5);
        let card_c = Card(Figure::Base(Suit::Diamond, Rank::Ace), 5);
        let card_d = Card(Figure::Trump(Trump::One), 45);
        let card_e = Card(Figure::Trump(Trump::TwentyOne), 45);
        let card_f = Card(Figure::Fool, 45);
        assert!(card_b > card_a);
        assert!(card_a > card_c);
        assert!(card_d > card_a);
        assert!(card_e > card_d);
        assert!(card_f < card_a);
    }

    #[test]
    fn test_card_unicity_in_hashmap()
    {
        let mut cards = HashSet::new();
        cards.insert(Card(Figure::Base(Suit::Spade, Rank::Ace), 5));
        cards.insert(Card(Figure::Trump(Trump::One), 45));
        cards.insert(Card(Figure::Fool, 45));
        // voluntarily trying to add an already existing card
        cards.insert(Card(Figure::Base(Suit::Spade, Rank::Ace), 5));
        assert!(cards.len() == 3);
    }
}