#![allow(dead_code)]
#![allow(unused_assignments)]
// Lifetimes
// lifetimes are special to rust and their job is 
// to specify the relationship between multiple references that affect each other
// Here are syntax examples: 
// &i32  = a regular reference
// &'a i32 = a reference with explicit lifetime
// &'a mut i32 = a mutable reference with explicit lifetime
// -----------------------------------------------------------
// How do I know where to use lifetime specifiers? 
// 1. You are returning a reference, that is dependent on parameters (such as function 'longer')
// 2. You have multiple references and the compiler needs to know their relationship from the point of lifetime
// 3. explicitly stating the lifetime for improving readability and accurate lifetime check

// lifetime specifiers in structs.
mod person;

// Here we define a lifetime 'a and so the function must return a type '&'a string'
// the reason why we need these specifiers is, that we need to identify how long will these parameter live
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    } 

    y
}


fn checkout(string1: &str) {
    let string2 = "efgh";

    println!("The longer string is {}", longer(&string1, &string2))
}


fn main() {
    let mut string1 = String::from("abcd").as_str();

    {
        let x = "xyz";
        
        // This is example from the book, but I fixed it so, that our program can compile.
        // it's fixed by taking ownership of the "xyz"
        string1 = x;
    }

    checkout(&string1);

    let shared_surname = String::from("Jones");
    
    let parent = person::Person { 
        first_name: String::from("Peter"), 
        last_name: shared_surname.as_str(), 
        age: 45
    };

    let son = person::Person::new(
        String::from("Andrew"), 
        parent.last_name, 
        22
    );

    println!("{}", parent.to_string());
    println!("{}", son.to_string());
}
