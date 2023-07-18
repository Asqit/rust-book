use self::back_of_house::Appetizer;

// While this modules is public, the "front_of_house" is not.
// we cannot access it's members nor target the module itself.
// the following test::greet would fail
pub fn greet() {
    crate::front_of_house::hosting::seat_at_table();

    // Even though we bring the 'front_of_house'
    // into our scope, we can't access it's private fields
    // meaning we are not calling greet function of 'font_of_house'
    // but our test::greet()
    use crate::front_of_house;

    // Error: Can't access private prop
    // front_of_house::greet();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("Peaches") 
            }
        }
    }

    // In contrast to the structs
    // if we make enum public, all of it's cases are 
    // public 
    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    
    // We can't access private property
    // If we really want to give users a way to access it 
    // we need to make it public in the struct definition.
    // For now, this causes error.
    // --------------------------------------------------
    // meal.seasonal_fruit = String::from("Raspberries");

    // See? We have access to every variant of enum when it's public
    let appezier1 = back_of_house::Appetizer::Salad;
    let Appetize2 = back_of_house::Appetizer::Soup;

    let link = "https://localhost:8080";
}