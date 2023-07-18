
#[allow(dead_code)]

/// represents a `32bit` floating point vector
struct Vector2f {
    x: f32,
    y: f32,
}

// Instance method (accessed by '.')
impl Vector2f {
    fn Vector2f(x: f32, y: f32) -> Self {
        Self { x, y }
    } 

    fn distance_to(&self, target: &Vector2f) -> f32 {
        let x = self.x - target.x;
        let y = self.y - target.y;

        (x * x + y * y).sqrt()
    }

    fn scale(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }

    fn is_bigger(&self, vector: &Vector2f) -> bool {
        self.x > vector.x && self.y > vector.y
    }
}

// Associated functions (accessed by '::')
impl Vector2f {
    fn distance_between(a: &Vector2f, b: &Vector2f) -> f32 {
        let x = a.x - b.x;
        let y = a.y - b.y;

        (x * x + y * y).sqrt()
    }
}

fn main() {
    let mut plane: Vector2f = Vector2f{x: 23.3, y: 392.0};
    let destination: Vector2f = Vector2f{x:32.0, y:393.5};

    plane.scale(1.2);

    Vector2f::distance_between(&plane, &destination);

    println!(
        "{} is distance between plane and it's destination",
        plane.distance_to(&destination)
    );
}