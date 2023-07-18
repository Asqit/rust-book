use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!(" Guess the number game");
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=");
    
    loop {
        println!("Remember, that any non numerical value will quit the app.");
        println!("Enter your guess:");

        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Giving up ?");
                println!("Righty then, the secret number was {}", &secret_number);
                break;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Congratulations, You Win!");
                break;
            }
        };
    }
}