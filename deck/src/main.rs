use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let mut cards = vec![]; // Vec::new()

        // Make list of suits
        let suits = vec![
            "♠", "♥",
            // "♦", "♣"
        ];

        // Make list of value
        let values = vec![
            "A", "K", "Q",
            // "J", "10", "9", "8", "7", "6", "5", "4", "3", "2"
        ];

        // Make a loop to add the cards
        for suit in suits {
            for value in &values {
                let card = format!("{}{}", value, suit);
                cards.push(card);
            }
        }

        Self { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck: Deck = Deck::new();
    deck.shuffle();
    println!("Here is your hand: {:#?}", deck.deal(4));
    println!("Here is your deck: {:#?}", deck);
}
