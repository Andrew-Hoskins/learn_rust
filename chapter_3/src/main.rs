fn main() {
#![allow(unused)]
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
println!("{c}, {z}");

//Tuple Type
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (e, f, g) = tup;
//let five_hundrered = e.0;
println!("The value of f is {f}");

// Array Type
let months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
// Access Arrays
let one = months[0];
println!("{one}");

//In function signatures, you must declare the type of each parameter.
fn another_function(value: i32, label: char) {
    println!("The measurement is: {value}, {label}");
//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value.

//x: i32 declares the parameter type.
//-> i32 declares the return type.
// fn plus_one(x: i32) -> i32 {
//     x + 1 //No semi-colon for an expression
// }

// fn main() {
//     let x = plus_one(5);
//     println!("The value of x is: {x}");
// }
}
