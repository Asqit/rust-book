![allow(dead_code)]

// Null in Rust
// Rust, unlike most languages, does not have "Null Reference Type" instead it uses special enum call `Result`
// This enum has only two options: 1. `None` and `Some(T)`. 
// For example you might have a function, that might return Null Pointer in Java you would have to write a try & catch block 
// to catch potential NullPointer exception. In Rust you get this enum. 

fn main() {
    let name: String = String::from("Nazir");

    // Here we are trying to access character at position 6, but the name variable has only 5 character
    // In some languages, we would get something like "IndexOutOfBounds" exception or we would get `null`
    // In Rust we can prevent that by using the "Option". The generic of `Some(T)` is our result
    // and in case of null we get `None`
    println!(
        "Let's catch potential index out of bounds exception! Character at position 6 {} <--", 
        match name.chars().nth(6) {
            Some(exists) => exists,
            None => "Ooops, there is no character at pos 6.".toString()
        }
    );
}
