// In normal manner, this module is private 
// But function "eat_at_restaurant" can access this module
// because it's sibling. Siblings can see each others, however
// we cannot see their inner parts unless they are public.
// e.g. greet function is not visible, while the other public
// members are.
mod front_of_house {
    fn greet() {
        // We can also use relative path inside of our module 
        // by using the 'self' keyword. It's similar to 'this' in java
        // or './' in file-system.
        self::serving::take_order();

        // We can access parent module by calling 'super'
        // it's like '..' in file-system.
        super::eat_at_restaurant();
    }

    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {
            super::serving::take_order();
        }
    }

    pub mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        pub fn take_payment() {}
    }
}

mod test;

// We can shorten our way to access module items 
// by using keyword 'use'. It's just like C++ 'using' keyword
// 'using std::String' -> 'use std::String'
use crate::test::*;

// We can also import as 
use crate::test::eat_at_restaurant as ear;

pub fn eat_at_restaurant() {
    // Absolute path to module function
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // The code fails to compile, as this 
    // module function is trying to access private module
    // "front_of_house".
    crate::test::greet();

    // See? Now with 'use crate::test::*' we don't 
    // have to specify entire path
    // 'crate::test::greet()' -> 'greet()'
    greet();

    // See? We used custom identifier to access a function
    // in different namespace.
    ear();
}
