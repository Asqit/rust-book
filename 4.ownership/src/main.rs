// Slices 
// slices are immutable reference variables, that points to certain 
// part of a collection. (Like a string is made up of characters)


// We are taking &str as parameter, 
// because it's compatible with both &String and &str.
fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()  {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let sentence: String = String::from("Ahoj, světe!");
    let greetings: &str = first_word(&sentence);

    println!("{greetings}");

    let mut sentence: String = sentence.to_uppercase();

    println!("{sentence}");

    println!("{greetings}");

    sentence.clear();

    println!("{sentence}");

    // The reason why we are still able to see the original "Ahoj"
    // here, is because we shadowed the original sentence variable.
    // but that doesn't mean we destroyed it. 
    // It still exists in the memory, up until the code-block ends 
    // then it's dropped by Drop function
    println!("{greetings}");
}