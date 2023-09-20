#[allow(dead_code)]

// Errors 
// Rust differs between two types of errors. First are panics or error type that causes the application to stop working. 
// Second are `Result` enum which are lot a like a exception in Java. 
// By the official doc, we refer to these as "Unrecoverable" and "Recoverable" errors.

// Panics (Unrecoverable errors) ------------------------------------------------------------------------------------------
// are type of errors, that can be caused either by explicitly or by taking action that causes our code to panic. 
// By default panic will: 
// 1. print the error message
// 2. unwind 
// 3. clear after the program
// 4. quit
//
// Because the whole "cleaning after program" is quite demanding and time consuming, we can leave that to the underlying OS.
// by simply adding two lines into the compiler's `Cargo.toml` file.
// 
// Cargo.toml:
// [profile.release]
// panic = 'abort'

/// Simple function, that causes panic 
fn cause_panic() {
    panic!("Ha ha I'm in danger.");
}


// Result (Recoverable errors) ------------------------------------------------------------------------------------------
// These the the types of errors that are minor. For example if file is missing we might create it instead of panicking.
// Instead of using exceptions and messing the codebase by use of try & catch block
// the rust uses an enum type (Result), that we can simply match. 
// Here is how the enum is defined: 
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
//}

use std::{fs::File, io::Read};

fn open_file() {
    
    // The result of File::open is Result<File, Error>
    let greetings_file_result = File::open("hello.txt");


    // Now either will receive a panic!()
    // or the resulting desired file.
    let file = match greetings_file_result {
        Ok(file) => file,
        Err(err) => panic!("Failed to open a file {:?}", err)
    };

    // In the future we will learn to use closures
    // Those are used to reduce this match madness 

    // We can also use ".expect(&str)" or ".unwrap()"
    // Expect method will throw panic with customized message
    // whereas the unwrap will only cause panic. 
    // both, if successfull, will return the desired Ok(value) => value 
}


fn get_username_from_file(filename: &str) -> Result<String, std::io::Error> {
    let mut username: String = String::new();

    // We can use shortcut to propage error with "?"
    // The "?" will return Err with type matching the function signature (std::io::Error in our case)
    // and if successful, then we'll get the Ok(T) => T
    File::open(&filename)?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    let username = get_username_from_file("hello.txt");

    print!("{}", match username {
        Ok(value) => value,
        Err(_) => "eee".to_string()
    });
}
