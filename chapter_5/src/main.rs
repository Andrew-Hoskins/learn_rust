struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Example Program Using Structs
fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

//Refactoring with Tuples
fn main() {
    let rectangle = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rectangle)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
