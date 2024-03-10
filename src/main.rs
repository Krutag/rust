use rand::Rng;
use std::io::{self,Write};
use std::cmp::Ordering;

const MAX_NUMBER: i32 = 100;
const MAX_ATTEMPT: i32 =10;

enum State {
    TooHigh,
    TooLow,
    Won,
    Lost
}

struct GuessGame {
    secretenum: i32,
    attempt: i32,
}

impl GuessGame {
    fn guess(&self, number:i32) -> State {
        if self.attempt == MAX_ATTEMPT {
            return State::Lost;
        }
        match number.cmp(&self.secretenum) {
            Ordering::Less => State::TooHigh,
            Ordering::Greater => State::TooHigh,
            Ordering::Equal => State::Won,
        }
    }
}

fn main() {
    let mut game = GuessGame {
        secretenum: rand::thread_rng().gen_range(0,MAX_NUMBER),
        attempt: 0,
    };

    println!("WELL COME TO THE GUESSING GAME OF KRUTAGNA");

    loop{
        print!("ATTEMPT {:?} of {:?}: ",game.attempt,MAX_ATTEMPT);
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("FAILED TO READ INPUT");

        let number: i32 = input.trim().parse().expect("INPUT NOT A NUMBER");
        game.attempt += 1;

        match game.guess(number) {
            State::TooLow =>println!("TOO LOW"),
            State::TooHigh => println!("TOO HIGH"),
            State::Won =>{
                println!("YOU WIN!");
                break;
            }
            State::Lost => { 
                println!("YOU LOST. THE NUMBER WAS : {}",game.secretenum);
                break;
            }
        }
    }


}
