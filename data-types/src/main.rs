#[allow(dead_code)]
fn main() {
    // Type system 
    // Rust, just like C/C++, objective-C and Java 
    // is statically typed language
    // meaning everything has certain type
    // Yes the language can infer the type, just like JS/RB/LUA...
    // But unlike these you can't reassign different data type to variable,
    // mismatch the function parameter type or others
    // ----------------------------------
    // Type annotation works like in TypeScript 
    // e.g. let x: number; == let mut x: i32;
    // or function getX(): number {} == fn getX() => i32 {}

    // Scalar types
    // by 'scalar' types we mean types that represents a single value
    // rust knows: chars, ints, floats, booleans
    // -----------------------------------
    // Integers 
    // just like anywhere else, integer is just a number without fractional component
    // rust knows integers from 8bit up to 128bit and special arch
    // they are differentiated by signed and unsigned respectively: 
    // SIGNED = i8, i16, i32, i64, i128, isize (the special arch)
    // UNSIGNED = u8, u16, u32, u64, u128, usize (the special arch)
    // ------------------------------------
    // Floating point numbers 
    // Again, like integers, floats can vary from 8bits up to 128bit and special arch
    // Here are example operations with floats: 
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Booleans 
    // booleans are the simplest of them all 
    // just like everywhere else, boolean is either true or false
    let truthy: bool = true;
    let falsy: bool = false;

    // Characters 
    // Just like in Java, C/C++... character represents a single character
    // and are marked with single quotes 
    let x: char = 'x';

    // Tuples 
    // Rust also knows tuples
    // usually a tuple is a way of grouping together a number of values with variety of types into one
    let tuple_example: (&str, i32) = ("Andrew", 21);
    let (name, age) = tuple_example;
    print!("Hi, my name is {} and I am {age} years old", tuple_example.0); 


    // Arrays 
    // just like in C/C++, Java...arrays are structure of one or more elements with same type 
    // grouped into single variable. Arrays have fixed length and if you try to access beyond it
    // rust will throw a panic. Usually language like C/C++ allows you to do this dangerous behavior
    // rust will panic and so you can't access foreign memory.
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    const ARRAY_OF_FIVE_OF_THREES: [i32;3] = [5;3];

}
