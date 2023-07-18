// Importing the standard I/O library
use std::io;
use rand::Rng;

// Program's entry point function.
fn main() {

    // Giving user instructions into console window
    println!("Guess the number!");

    loop {
        println!("Enter your guess.");
        
        // Creating a new instance of 'ThreadRng' and utilizing it 
        // in 'gen_range' method. 
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        // Creating a mutable variable 
        // called guess with type of 'String'
        // By default variables are immutable
        // to be actually able to change their contents
        // you need to put the 'mut' keyword in front of 'let'
        // -------------------------------------------
        // Side-notes
        // - Strings are encoded in UTF-8 EMOJIS
        // - Type definition are same as in TypeScript 
        let mut guess: String = String::new();

        // Read user input and update the 'guess' variable of the user's input
        // The 'io::stdin()' gives us access to object/instance of 'std::io::Stdin'
        // which then gives us access to all sorts of input methods and variables.
        // -----------------------------------------------------------------------
        // the '&' tells us, that the following parameter is reference and just like vars 
        // even reference are immutable by default so we need to add 'mut'
        // By using references, instead of copying the 'guess' value we save a bit of memory
        // -----------------------------------------------------------------------
        // The 'expect' method is just like 'catch' in try & catch block
        // It allows us to fail-safely. 
        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read the line");


        // Because we initially set the 'guess' variable to be String
        // have to somehow get it's numerical value from the string, so that it can be compared
        // with the 'secret_number'. 
        // Here we used thing called shadowing. Shadowing allows us the 'redefine' a variable
        // so that we don't have typically create two variables (guess_string, guess_number)
        // --------------------------------------------------------------------------------
        // We also manage parsing errors by using 'match' keyword
        // Basically we ignore all errors and quit the app if any occurs
        // and if parsing succeeded we return the parsed number
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => break,
        };

        // Match expression. Match is very much like switch in other languages
        // The 'guess.cmp' function returns an 'std::cmp::Ordering' enum and the match
        // will match the result to correct hand we defined in the code block. 
        // e.g. guess = 3, secret_number = 3
        // guess.cmp(secret_number) -> std::cmp::Ordering::Equal 
        // ----------------------------------------------------------------------
        // match will then go thru the defined code block and check each "hand"
        // until it finds 'std::cmp::Ordering::Equal'
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Greater => print!("Too Big"),
            std::cmp::Ordering::Less => print!("Too Small"),
            std::cmp::Ordering::Equal => {
                print!("You Win!");
                break;
            },
        }

        // Printing what user's previously entered number
        println!("You have entered: {}", guess);
    }
}

