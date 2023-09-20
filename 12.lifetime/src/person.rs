// Lifetime specifiers inside of structures
pub struct Person<'a> {
    pub first_name: String,
    pub age: i32,

    // When using references inside of a struct we need to use lifetime specifiers
    // just as we did here.
    pub last_name: &'a str
    
}

// We can also specify lifetime inside of methods & functions
impl<'a> Person<'a> {

    // Here we specify a lifetime for the surname of the newly constructed 
    // person. Basic idea goes like this: the surname lives as long as the last child
    pub fn new(first_name: String, last_name: &'a str, age: i32) -> Self {
        Self {
            first_name,
            last_name,
            age
        }
    }

    pub fn to_string(&self) -> String {
        let result = format!(
            "{} {} is {} years old",
            self.first_name,
            self.last_name,
            self.age
        );

        String::from(result)
    }
}