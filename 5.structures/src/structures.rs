// Example of unit structure
struct AlwaysEqual;

// Example of tupple structures
struct Vector2f(f32, f32);
struct Rgb(u8, u8, u8);

// Example of structureS
struct User {
    is_active: bool,
    username: String,
    email: String,
    direction: Vector2f,
    favorite_color: Rgb,
}

// Function, that returns a new User structure
fn register_new_user(username: String, email: String) -> User {
    User {
        favorite_color: Rgb(0, 0, 0),
        direction: Vector2f(0.0, 0.0),
        is_active: true,
        username,
        email,
    }
}

fn main() {
    let andy: User =
        register_new_user(String::from("Andy"), String::from("ondrejtucek9@gmail.com"));

    // Similarly to javascript we can use spread/range operator
    // to copy values of previous struct instance.
    let updated_andy: User = User {
        email: String::from("ondrejtucek9@seznam.cz"),
        direction: Vector2f(1.23, 0.0),
        ..andy
    };

    // Please note, that all struct properties, that does not implement "Copy" trait
    // cannot be accessed anymore, because their ownership changed to 'update_andy'
    // Thus the following commented function call would fail to compile.
    // println!("{}", andy.username);

    println!("The original email is: {}", andy.email);
    println!(
        "The user has been updated of email address: {}",
        updated_andy.email
    );
}
