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

#[derive(Copy, Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    // Создает новую карту с указанными rank и suit
    pub fn new(rank: Rank, suit: Suit) -> Card { // static method not use &self
        Card { rank, suit }
    }

    // Выводит читаемый формат для карты
    pub fn render(&self) -> String { // method use &self
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
        Rank::King => "K",
    };
}
