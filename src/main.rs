use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    fn print(x: &str) {
        println!("🦀 {x}")
    }

    print("Welcome to the guessing game, I'm your host, Ferris.");
    print("Im thinking of a number between...");

    let number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input your number.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("🦀 I cant read that!");

        let guess: u32 = guess.trim().parse().expect("Please type a number.");

        println!("You guessed {guess}");

        match guess.cmp(&number) {
            Ordering::Less => print("Too small!"),
            Ordering::Greater => print("Too big!"),
            Ordering::Equal => {
                print("You did it!");
                break;
            }
        }
    }


}