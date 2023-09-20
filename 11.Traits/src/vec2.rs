pub struct Vec2<T> {
    x: T,
    y: T
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// We can also expose method only for if the type has required traits
// for example here we require traits 'std::cmd::PartialOrd' and 'Copy'
impl<T: std::cmp::PartialOrd + Copy> Vec2<T> {
    fn greater_value(&self) -> T {
        if self.x > self.y {
            return self.x
        } 

        self.y
    }
}