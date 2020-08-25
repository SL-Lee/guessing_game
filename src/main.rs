use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Ok(guess) => guess.value,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                },
            },
            Err(_) => {
                println!("Error: Guess must be a valid integer between 1 and 100.");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Result<Guess, &'static str> {
        if value < 1 || value > 100 {
            return Err("Guess value must be between 1 and 100.");
        }

        Ok(Guess { value })
    }
}
