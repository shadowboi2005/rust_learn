use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number");
    let secretnum:i32 = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guessval:i32 = guess.trim().parse().expect("Type a number!");

        match guessval.cmp(&secretnum) {
        Ordering::Equal => {println!("you guessed right: {guess}");break;},
        Ordering::Less => println!("Guessed too less!"),
        Ordering::Greater => println!("Guessed too high!"),
        }
    }
}
