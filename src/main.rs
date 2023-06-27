mod deck;

fn main() {
    let deck = deck::create_desk();

    println!("Hello, world!");

    let deck = deck::shuffle(&deck);

    for card in deck {
        println!("{}", card.render());
    }
}
