mod deck;

fn main() {
    let deck = deck::create_desk();

    println!("Hello, world!");

    let deck = deck::shuffle(&deck);

    for card in deck {
        println!("{}", card.render());
    }
}

pub mod black_jack {
    use std::io;

    use crate::deck::{Card, Deck};

    struct User {
        name: String,
    }

    struct Player {
        user: User,
        cards: Vec<Card>,
    }

    impl Player {
        fn new(name: String) -> Player {
            return Player {
                user: create_user(name),
                cards: vec![]
            }
        }

        fn add_card(&mut self, card: Card) {
            self.cards.push(card);
        }

        pub fn get_card(&mut self, mut deck: Deck) {
            match deck.give_cards() {
                Some(card) => self.add_card(card),
                None => {}
            }
        }
    }

    enum GameStatus {
        Running,
        Finished,
    }

    fn create_user(name: String) -> User {
        return User { name };
    }

    fn give_player_card(player: &mut Player, deck: &mut Deck) {
        match deck.give_cards() {
            Some(card) => player.add_card(card),
            None => (),
        }
    }

    pub fn init() {
        greeteng();

        let mut deck = Deck::new();

        let mut player = Player::new(String::from("player"));
        let mut dealer = Player::new(String::from("dealer"));

        give_player_card(&mut player, &mut deck);
        give_player_card(&mut dealer, &mut deck);
        give_player_card(&mut player, &mut deck);
        give_player_card(&mut dealer, &mut deck);
    }

    pub fn run() {
        init();

        loop {
            let answer = get_user_input();
            break;
        }
    }

    fn greeteng() {
        println!("Welcome to game 'Black Jack'");
    }

    fn get_user_input() -> String {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        return input.trim().to_string();
    }
}
