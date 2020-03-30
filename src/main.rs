use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut guess_count = 0;
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Error! '{}' resulted in error message: '{}'", guess.trim(), error);
                continue;
            }
        };
        guess_count += 1;
        println!("You guessed: {}. You have made {} guesses", guess, guess_count);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win! It took you {} guess(es)", guess_count);
                break;
            }
        }
    }
}
