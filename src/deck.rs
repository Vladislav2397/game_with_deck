// Deck = Колода
// Suit = Масть
// Rang = Ранг карты
// Black Jack = 21 очко

use crate::card::{Card, Suit, Rank};
use rand::{seq::SliceRandom, thread_rng};

#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let deck = create_desk();
        let deck = shuffle(&deck);

        return Deck { cards: deck };
    }

    pub fn length(&self) -> usize {
        return self.cards.len();
    }

    // pub fn give_cards(&mut self) -> Option<Card> {
    //     return self.cards.pop();
    // }
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
        Rank::King,
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
pub fn create_shuffle_desk() -> Deck {
    return Deck::new();
}

// [PURE] Возвращает перетасованную колоду из исходной
pub fn shuffle<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut newvec = vec.to_vec();

    newvec.shuffle(&mut thread_rng());

    newvec
}

// [PURE] Возвращает карту и новую колоду без этой карты
pub fn pop_card(deck: &Deck) -> (Card, Deck) {
    let mut cards = Vec::clone(&deck.cards);
    let new_card = cards.pop().unwrap();

    return (new_card, Deck {
        cards,
    });
}