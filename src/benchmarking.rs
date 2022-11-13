use crate::{User, Deck, Hand, Result};
use std::time::Instant;
use std::io::{stdout, Write};

fn time_to_blackjack() {
    let user = User::new("User", 0, false);
    let instant = Instant::now();
    let mut attempts: u64 = 0;

    while !user.hand.blackjack {
        attempts += 1;
        let mut shoe = Deck::new(1);
        shoe.shuffle(0);
        let mut user = User::new("User", 0, false);
        user.hand = Hand::new();
        shoe.draw(Some(&mut user), 2);
    }
    user.hand.print_information(Some(&user));
    println!("Time elapsed: {:?}. Total iterations: {}", instant.elapsed(), &attempts);
}

fn test(iterations: u64, verbose: bool) -> [u64; 3] {
    const BEGINNING: &str = "~ [ ";
    const END: &str = " ] ~ ";
    const TICK: &str = "#";
    
    let mut wins: u64 = 0;
    let mut loss: u64 = 0;
    let mut push: u64 = 0;

    let i100 = (&(iterations as f64) / 100.0) as u64;
    let mut progress: u64 = 1;

    for i in 1..=iterations {
        let mut shoe = Deck::new(1);
        let mut user = User::new("test_user", 0, false);
        let mut dealer = User::new("dealer", 0, true);
        
        shoe.shuffle(0);
        shoe.draw(Some(&mut user), 2);
        shoe.draw(Some(&mut dealer), 2);

        match Result::compare(&user, &dealer) {
            Result::Win => wins += 1,
            Result::Loss => loss += 1,
            Result::Push => push += 1,
            Result::Surrender => loss += 1,
            Result::Blackjack => wins += 1,
        }

        if verbose && i == i100*progress {
            let mut progress_ticks = progress.clone();
            progress += 1;
            print!("\r{BEGINNING}");
            for _ in 0..100 {
                if progress_ticks != 0 {
                    print!("{TICK}");
                    progress_ticks -= 1;
                } else {
                    print!(" ");
                }
            };
            print!("{END}");
            stdout().flush().unwrap();
                
        };
    }
    println!("");
    
    let array: [u64; 3] = [wins, loss, push];
    return array;
}

pub fn choose_mode() {
    // WIP
}
