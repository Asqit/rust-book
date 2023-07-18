#[allow(dead_code)]

fn main() {
    // Mutability ------------------------------>
    // By design variables are immutable on default
    // To change this you need to use the 'mut' 
    // keyword. 
    // -------------------------------------------
    // Example of unsafe code
    // let x = 5;
    // println!("The value of x is {}", &x);
    // x = 6;
    // println!("The value of x is {}", &x);
    // -------------------------------------------
    // The very same code, but safe 
    // let mut x: i32 = 69;
    // println!("The value of x is {x}");
    // 
    // x = 420;
    // println!("The value of x is {x}...nice");


    // Constants ------------------------------>
    // The difference between regular variables 
    // and constants is that constants always 
    // will be immutable. (The mut keyword won't work)
    // And unlike vars, the type HAVE TO be annotated
    // bellow is correct example of constant
    // (UPPERCASE_SNAKE_CASE is preferred)
    const THREE_HOURS_IN_SECS: u32 = 60 * 60 * 3;

    // Showing --------------------------------->
    // Another difference between most languages 
    // and rust is shadowing mechanism.
    // This mechanism allows us to re-define a 
    // variable with different type or value
    let current_year: i32 = 2023;

    // Basically the second definitions 
    // overshadows the initial one. The compiler knows 
    // the differences. 
    let mut current_year: &str = "TWENTY-TWENTY_FOUR";

    // We can also change the mutability 
    current_year = "TWENTY_TWENTY_FIVE";

}
