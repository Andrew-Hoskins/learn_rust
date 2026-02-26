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

// Refactoring with Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {rect1:?}");
    println!(
        "The area if the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Method Syntax
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Implementation Block
impl Rectangle {
    fn area(&self) -> u32 {
        // 1st param is always self which represents the instance of struct
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectnagle is {} square pixels.",
        rect1.area()
    );
}

// Method Syntax
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Multiple impl Blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Methods with More Parameters
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect1.can_hold(&rect3));
}
