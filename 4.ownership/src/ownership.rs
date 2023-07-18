#[allow(dead_code)]
// Ownership rules: 
// 1) each value in rust has an owner
// 2) there can be only ONE owner at a time
// 3) when the owner gets out of scope, so is the value


// This function takes ownership of a String type value
// modifies it string value and returns the ownership back.
fn take_ownership(mut new_owner: String) -> String {
    println!("variable 'new_owner' just got a new value: {new_owner}");

    new_owner.push_str("used, good condition tho");

    return new_owner;
}

fn main() {
    // Hereby defined variable is currently the owner of 
    // it's value. This variable is valid here in this code block
    // after this codeblock the variable and it's value will be deleted
    // (resources released)
    // We can change that by passing the ownership to someone else
    let mut owner: String = String::from("BMW 328i");

    // Here we are not borrowing the ownership, that would be done through 
    // reference '&' notation. No, here we are passing the ownership to variable/parameter
    // called 'new_owner'
    // Don't worry about the assignment just yet, it's explained in the following comment.
    let tmp: String = take_ownership(owner);

    // When we passed the ownership, we also deleted the variable in this codeblock
    // meaning it cannot be used here ever again. 
    // To make it usable again, you need to get the ownership again.
    // To get the ownership back, in this example we get get it from the return statement
    // via assignment. 
    println!("{tmp}");

    // And here we re-used the original owner / identificator
    owner = tmp;

    // So here are back with the original.
    // That was way to much of a hustle to do this simple thing.
    // But it was only to explain how ownership works. 
    // To simplify all this we have mechanism called borrowing. (reference passing)
    println!("{owner}");
}
