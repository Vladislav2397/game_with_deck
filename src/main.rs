mod deck;
mod card;

fn main() {
    let game = black_jack::init();
    black_jack::run(game);
}

pub mod black_jack {
    use std::io;

    use crate::card::{Card};
    use crate::deck::{Deck, pop_card};

    #[derive(Clone, Debug)]
    struct User {
        name: String,
    }

    impl User {
        fn new(name: String) -> User {
            return User {
                name
            }
        }
    }

    #[derive(Clone, Debug)]
    struct Player {
        user: User,
        cards: Vec<Card>,
    }

    impl Player {
        fn new(name: String) -> Player {
            return Player {
                user: User::new(name),
                cards: vec![]
            };
        }

        fn show_cards(&self) {
            show_cards(&self.cards);
        }
    }

    fn show_cards(cards: &Vec<Card>) {
        let mut string = String::from("");

        for card in cards.iter() {
            string = string + &format!("{} ", card.render()).to_string();
        }

        println!("{}", string);
    }

    pub struct Game {
        dealer: Player,
        player: Player,
        deck: Deck,
    }

    impl Game {
        pub fn new() -> Game {
            return Game {
                player: Player::new(String::from("player")),
                dealer: Player::new(String::from("dealer")),
                deck: Deck::new(),
            }
        }

        pub fn init(&mut self) {
            let deck = Deck::new();

            let (dealer, deck) = deal_cards(&self.dealer, &deck);
            let (player, deck) = deal_cards(&self.player, &deck);

            self.update(player, dealer, deck);
        }

        fn update(&mut self, player: Player, dealer: Player, deck: Deck) {
            self.player = player;
            self.dealer = dealer;
            self.deck = deck;
        }
    }

    fn add_card_to_player(player: &Player, card: Card) -> Player {
        let mut new_player = player.clone();

        new_player.cards.push(card);

        return new_player;
    }

    fn deal_cards(player: &Player, deck: &Deck) -> (Player, Deck) {
        let mut new_player = player.clone();
        let mut new_deck = deck.clone();

        for _ in 0..2 {
            let (card, pop_deck) = pop_card(&new_deck);

            new_player = add_card_to_player(&new_player, card.clone());
            new_deck = pop_deck;
        }

        return (new_player, new_deck);
    }

    enum GameStatus {
        Running,
        Finished,
    }

    pub fn init() -> Game {
        greeteng();

        let mut game = Game::new();

        game.init();

        return game;
    }

    pub fn run(game: Game) {
        loop {
            let action = ask_want_get_card(&game.player);

            match action {
                GameAction::Invalid => continue,
                _ => break,
            }
            break;
        }
    }

    fn greeteng() {
        println!("Welcome to game 'Black Jack'");
    }

    enum GameAction {
        AddCard,
        StopDeal,
        Invalid,
    }

    fn ask_want_get_card(player: &Player) -> GameAction {
        println!("Your cards is:");
        player.show_cards();

        println!("What you want doing?");
        println!("1) Add card");
        println!("2) Stop");

        let answer = get_user_input();

        return match answer.as_str() {
            "1" => GameAction::AddCard,
            "2" => GameAction::StopDeal,
            _ => GameAction::Invalid,
        }
    }

    fn get_user_input() -> String {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        return input.trim().to_string();
    }
}
