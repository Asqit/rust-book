// Generics can also be used inside of structures

/// represents a simple 2d vector
struct Vec2<T> {
    x: T, 
    y: T
}


// Generics can also be used inside of struct or enum methods
// By say the '<T>' next to impl we are saying that this definition goes for 
// every generic type
impl<T> Vec2<T> {
    fn get_x<T>(&self) -> &T {
        &self.x
    }

    fn get_y<T>(&self) -> &T {
        &self.y
    }
}

// This is special, because we can also say, that we want to implement a method for specific type
// so for example, this method is only available for Vectors defined with 32bit floats
impl<f32> Vec2<T> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


struct Vec2B<T, U> {
    x: T,
    y: U
}

fn main() {
    let int_vec = Vec2 {x:5, y:10};
    let float_vec = Vec {x:5.5, y: 10.5};

    // This causes an error, because we've defined those properties to use
    // the same generic type for both 'x' and 'y'.
    // We can fix that by using more than one generic
    // see the Vec2B where we expect to have two different types
    let unknown_vec = Vec2 {x:5, y: 10.5 };

    // The program still crashes, but it is because of the upper variable
    // if you remove the previous and replace it with this the program will compile
    // and run just fine. 
    let special_vec = Vec2B {x:5, y: 10.5};
}