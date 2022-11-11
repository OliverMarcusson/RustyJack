struct User {
    id: u32,
    tokens: u32,
    hand: Hand
}

struct Hand {
    is_dealer: bool,
    cards: Vec<Card>,
    card_names: Vec<String>,
    hand_value: u32,
    blackjack: bool
}

impl Hand {
    fn new(is_dealer: bool) -> Hand {
        let hand: Hand = Hand { is_dealer: is_dealer, cards: Vec::new(), card_names: Vec::new(), hand_value: 0, blackjack: false };
        hand
    }

    fn draw(&self, deck: &mut Deck) {
        let card = deck.cards.pop();
        match card {
            None => None,
            Some(card) => {let card = Some(Card)},
        };
        
    }
}

struct Deck {
    cards: Vec<Card>,
    size: u8,
    cards_left: u32
}

impl Deck {
    fn new(size: u8) -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for i in 0..size {
            for j in 0..4 {
                for k in 1..=13 {
                    match j {
                        0 => cards.push(Card::make(k, Suite::Hearts)),
                        1 => cards.push(Card::make(k, Suite::Spades)),
                        2 => cards.push(Card::make(k, Suite::Diamonds)),
                        3 => cards.push(Card::make(k, Suite::Clovers)),
                        _ => unreachable!()
                    }
                    
                }
            }
        }
        let cards_left = &cards.len();
        let shoe: Deck = Deck { cards: cards, size: size, cards_left: *cards_left as u32 };
        shoe
    }

    fn print(deck: &Deck) {
        for i in deck.cards.iter() {
            println!("Card: {} of {}.", i.value_name, i.suite_name);
        }
    }
}

enum Suite {
    Hearts,
    Spades,
    Diamonds,
    Clovers,
}

struct Card {
    value: u32,
    suite: Suite,
    suite_name: String,
    value_name: String,
    clothed: bool,
    color: bool,
}

impl Card {
    fn make(val: u32, suite: Suite) -> Card {
        let mut card: Card = Card { value: val, suite: suite, suite_name: String::new(), value_name: String::new(), clothed: false, color: false };
        match val {
            11 => {
                card.value = 10;
                card.clothed = true;
                card.value_name = String::from("Knight");
            }
            12 => {
                card.value = 10;
                card.clothed = true;
                card.value_name = String::from("Queen");
            }
            13 => {
                card.value = 10;
                card.clothed = true;
                card.value_name = String::from("King");
            }
            1 => {
                card.value = 11;
                card.clothed = true;
                card.value_name = String::from("Ace");
            }
            _ => {
                card.value = val;
                card.clothed = false;
                card.value_name = val.to_string();
            }
        }

        match card.suite {
            Suite::Hearts => {
                card.color = true;
                card.suite_name = String::from("Hearts");
            }
            Suite::Spades => {
                card.color = false;
                card.suite_name = String::from("Spades");
            }
            Suite::Diamonds => {
                card.color = true;
                card.suite_name = String::from("Diamonds");
            }
            Suite::Clovers => {
                card.color = false;
                card.suite_name = String::from("Clovers");
            }
        }
        return card;
    }
}


fn main() {
    let shoe = Deck::new(1);
    Deck::print(&shoe);
}
