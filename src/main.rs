mod deck;
mod card;

fn main() {
    let mut game = black_jack::init();
    black_jack::run(&mut game);
}

pub mod black_jack {
    use std::io;

    use crate::card::{Card, Rank};
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

    fn get_card_total_value(player: &Player) -> i32 {
        let mut total_value = 0;

        for card in player.cards.iter() {
            total_value = total_value + get_card_value(&card);
        }

        return total_value;
    }

    fn get_card_value(card: &Card) -> i32 {
        return match card.rank {
            Rank::Two => 2,
            Rank::Ace => 10,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            _ => 0,
        }
    }

    fn is_player_value_big(player: &Player) -> bool {
        return get_card_total_value(player) > 21;
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

        pub fn give_card_player(&mut self) {
            let (card, deck) = pop_card(&self.deck);

            self.deck = deck;

            let player = add_card_to_player(&self.player, card);

            self.player = player;
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

    pub fn run(game: &mut Game) {
        loop {
            show_player_info(&game.player, &game.dealer);

            if is_player_value_big(&game.player) {
                println!("Your value is so big. You are lose");
                break;
            }

            let action = ask_want_get_card(&game.player, &game.dealer);

            match action {
                GameAction::Invalid => continue,
                GameAction::AddCard => {
                    game.give_card_player();

                    continue;
                },
                _ => break,
            }
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

    fn show_player_info(player: &Player, dealer: &Player) {
        println!("Your has value {} for cards is:", get_card_total_value(&player));
        player.show_cards();

        println!("Dealer cards is:");
        println!("{} **", dealer.cards[0].render());
    }

    fn ask_want_get_card(player: &Player, dealer: &Player) -> GameAction {
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
