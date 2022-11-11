use rand::Rng;
use std::{time::Instant, cmp::Ordering};

struct Deck {
    cards: Vec<Card>,
    size: u8,
    cards_left: u32
}

impl Deck {
    fn new(size: u8) -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for _ in 0..size {
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

    fn shuffle(&mut self, mut iterations: u32) {
        let mut rng = rand::thread_rng();
        if iterations == 0 { iterations = 52; };
        
        for _ in 0..iterations {
            let index = rng.gen_range(0..self.cards.len());
            let popped_card = self.cards.pop(); 
            
            match popped_card {
                None => {
                    println!("Deck is empty.");
                    break
                },
                _ => {
                    let popped_card = popped_card.unwrap();
                    self.cards.insert(index, popped_card);
                } 
            }
        }
    }

    fn draw(&mut self, user: Option<&mut User>, amount: u32) -> Option<Vec<Card>> {
        match user {
            None => {
                let mut vec: Vec<Card> = Vec::new();
                for _ in 0..amount {
                    let card = self.cards.pop();
                    vec.push(card.unwrap());
                }
                Some(vec)
            },
            Some(ref _i) => {
                let user = user.unwrap();
                for _ in 0..amount {
                    let card = self.cards.pop();
                    user.hand.cards.push(card.unwrap());
                }
                user.hand.evaluate();
                None
            }
        }
    }

    fn print(&self) {
        println!("Deck information: Size = {}, Cards Left = {}\n", self.size, self.cards_left);
        for i in self.cards.iter() {
            println!("Card: {} of {}.", i.value_name, i.suite_name);
        }
    }

    fn to_string(vec: &Vec<Card>) -> String {
        let mut string = String::new();
        for i in vec.iter() {
            string.push_str(i.value_name.as_str());
            string.push_str(", ");
        }
        string
    }
    
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

enum Suite {
    Hearts,
    Spades,
    Diamonds,
    Clovers,
}

// struct Users {
//     users: Vec<User>
// }

struct User {
    id: String,
    tokens: u32,
    hand: Hand,
    is_dealer: bool
}

impl User {
    fn new(name: &str, mut tokens: u32, is_dealer: bool) -> User {
        if tokens == 0 { tokens = 1000 };
        let user = User {id: String::from(name), tokens: tokens, hand: Hand::new(), is_dealer: is_dealer};
        user
    }

    fn return_hand(&self) -> String {
        let mut hand = String::new();
        for name in self.hand.card_names.iter() {
            hand.push_str(format!("{}, ", name).as_str());
        }
        hand.to_string()
    }
}

struct Hand {
    cards: Vec<Card>,
    card_names: Vec<String>,
    hand_value: u32,
    has_clothed: bool,
    blackjack: bool,
    surrender: bool
}

impl Hand {
    fn new() -> Hand {
        let mut hand: Hand = Hand { cards: Vec::new(), card_names: Vec::new(), hand_value: 0, has_clothed: false, blackjack: false, surrender: false};
        hand.evaluate();
        hand
    }

    fn evaluate(&mut self) {
        self.card_names.clear();
        self.hand_value = 0;
        self.blackjack = false;

        for card in self.cards.iter() {
            self.card_names.push(format!("{} of {}", card.value_name, card.suite_name));
            self.hand_value += card.value;
            if let true = card.clothed { self.has_clothed = true; }
        }
        if self.hand_value == 21 && self.has_clothed == true { self.blackjack = true; }
    }

    fn print_information(&self, user: Option<&User>) {
        if let Some(_i) = user {
            let hand = user.unwrap().return_hand();
            println!("{} Hand information:\nCards: {}\nValue: {}\nHas Clothed Card: {:?}\nHas Blackjack: {:?}", user.unwrap().id.as_str(), hand, self.hand_value, self.has_clothed, self.blackjack);
        } else {
            println!("Anonymous Hand information:\nValue: {}\nHas Clothed Card: {:?}\nHas Blackjack: {:?}", self.hand_value, self.has_clothed, self.blackjack);
        }
    }
}

enum Result {
    Win,
    Loss,
    Blackjack,
    Push,
    Surrender
}

impl Result {
    fn to_string(&self) -> String {
        match self {
            Result::Win => String::from("win"),
            Result::Loss => String::from("loss"),
            Result::Blackjack => String::from("blackjack"),
            Result::Surrender => String::from("surrender"),
            Result::Push => String::from("push"),
        }
    }
}

fn compare(user: &User, dealer: &User) -> Result {
    let user_hand_value = user.hand.hand_value;
    let dealer_hand_value = dealer.hand.hand_value;
    
    if let true = user.hand.surrender {
        return Result::Surrender;
    }

    if dealer_hand_value > 21 {
        return Result::Win;
    }

    if user_hand_value > 21 {
        return Result::Loss;
    }

    if user.hand.blackjack && !dealer.hand.blackjack {
        return Result::Blackjack;
    }

    match user_hand_value.cmp(&dealer_hand_value) {
        Ordering::Equal => Result::Push,
        Ordering::Greater => Result::Win,
        Ordering::Less => Result::Loss,
    }
}

fn time_to_blackjack() {
    let mut user = User::new("User", 0, false);
    let mut shoe = Deck::new(1);
    let instant = Instant::now();
    let mut attempts: u64 = 0;

    while !user.hand.blackjack {
        attempts += 1;
        shoe = Deck::new(1);
        shoe.shuffle(0);
        user.hand = Hand::new();
        shoe.draw(Some(&mut user), 2);
    }
    user.hand.print_information(Some(&user));
    println!("Time elapsed: {:?}. Total iterations: {}", instant.elapsed(), &attempts);
}

fn test() -> String {
    let mut shoe = Deck::new(1);
    shoe.shuffle(0);
    let mut user = User::new("test_user", 0, false);
    let mut dealer = User::new("dealer", 0, true);
    shoe.draw(Some(&mut user), 2);
    shoe.draw(Some(&mut dealer), 2);
    user.hand.print_information(Some(&user));
    dealer.hand.print_information(Some(&dealer));
    compare(&user, &dealer).to_string()
    
}

fn main() {
    println!("{}", test().as_str());
}
