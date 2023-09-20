#[allow(dead_code)]

// Generics
// generics is a technique and tool for reducing code redundancy. 
// a generic type can represent multiple different types, so that the function that utilize it
// can be used with more types and shouldn't be replicated for special ones.


// Example of removing code redundancy with generics
// take these two functions for example
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list.get(0).unwrap();

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    *largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest: &char = list.get(0).unwrap();

    for item in list {
        if item > largest {
            largest = item
        }
    }

    *largest
}

// we can get rid of those two functions and use only one 
// we only change the typing not the body.
// since we can't compare all types by ">, <, <=, >=..." we have to implement
// a limitation that helps us by limiting the types the function can accept.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![1,4,2,5,2,6,5,7,5,77,3,55,2,102,42];
    let characters = vec!('A', 'a', 'b', 'f'); 

    println!(
        "largest 32bit int: {}",
        largest_i32(&numbers)
    );

    println!(
        "largest char: {}",
        largest_char(&characters)
    );

    println!();
    println!("Now with generics");
    println!();

    println!(
        "largest 32bit int: {}",
        largest(&numbers)
    );

    println!(
        "largest char: {}",
        largest(&characters)
    );

    // See the results?
    // they are the same...so we don't really need the two previous functions
}
