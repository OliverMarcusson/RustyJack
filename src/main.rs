// struct Deck {
//     cards: Vec<Card>,
//     size: u8
// }

struct Card {
    value: u32,
    suite: Suite,
    suite_name: String,
    value_name: String,
    clothed: bool,
    color: bool,
}

enum Suite {
    Hearts,
    Spades,
    Diamonds,
    Clovers,
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
    let card = Card::make(11, Suite::Hearts);
    println!("Card suite: {}", card.suite_name)
}
