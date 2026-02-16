fn main() {
    // Variables are mutable by adding mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants are always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three Hours in Seconds equals: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // This is different than using mut, because we need to use let again to avoid compile time errors.
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {y}");

    let input = "42";
    let guess: u32 = input.parse().expect("Not a number"); // requires type annotation u32
    println!("{guess}");

    //Data Types = integers, floating-point numbers, Booleans, and characters.
    //Integer types default to i32
    // signed = positive or negative
    // unsigned = only positive.
    let v = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Booleans
    let t = true;
    let f: bool = false; // With explicit type annotation.
    println!("{v},{y}, {t}, {f}");

    // Character Type
    let c = 'z';
    let z: char = 'z'; // With explicit type annotation use single quotes for char.
    println!("{c}, {z}")
}
