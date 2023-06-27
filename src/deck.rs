// Deck = Колода
// Suit = Масть
// Rang = Ранг карты
// Black Jack = 21 очко

use rand::{thread_rng, seq::SliceRandom};

#[derive(Copy, Clone, Debug)]
pub enum Rank {
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

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    // Создает новую карту с указанными rank и suit
    fn new(rank: Rank, suit: Suit) -> Card {
        Card {
            rank,
            suit,
        }
    }

    // Выводит читаемый формат для карты
    pub fn render(&self) -> String {
        return format!("{}{}", rank_emoji(self.rank), suit_emoji(self.suit));
    }
}

// [PURE] Возвращает emoji вариант для suit
fn suit_emoji(suit: Suit) -> &'static str {
    return match suit {
        Suit::Hearts => "❤️",
        Suit::Diamonds => "♦️",
        Suit::Clubs => "♣️",
        Suit::Spades => "♠️",
    };
}

fn rank_emoji(rank: Rank) -> &'static str {
    return match rank {
        Rank::Ace => "A",
        Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K"
    }
}

// [PURE] Создает новую колоду из 52 карт
pub fn create_desk() -> Vec<Card> {
        let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        let ranks = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King
        ];
        let mut deck = Vec::new();
    
        for suit in suits.iter() {
            for rank in ranks.iter() {
                let card = Card::new(*rank, *suit);
                deck.push(card);
            }
        }
    
        return deck;
}

// [PURE] Возвращает новую перетасованную колоду
pub fn shuffle<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut newvec = vec.to_vec();

    newvec.shuffle(&mut thread_rng());
    
    newvec
}