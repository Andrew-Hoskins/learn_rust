// 4.2. References and Borrowing
fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{s}");

    let s1 = String::from("hello");
    let s2 = s1; // copies the pointer, length, and capacity to s2
    println!("{s2}, world"); //s1 is out of scope so will error

    // Scope and assignement
    let mut a = String::from("Hello!");
    a = String::from("Ahoy!"); // new string on the heap containing ahoy
    println!("{a}, world!") // old string hello is dropped ana a new pointer is to ahoy
}

fn refrences() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //These ampersands represent references

    println!("The length of '{s1}' is {len}");
}

fn calculate_length(s: &String) -> usize {
    // s: is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.

// At any given time, you can have either one mutable reference
// Or any number of immutable references. References must always be valid.
fn mutable_refs() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem as both immutable refs
    let r2 = &s;
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem, create a mutable reference
    println!("{r3}");
}

// A string slice is a reference to part of a string.
fn slices() {
    let s = String::from("Hello World");

    let hello = &s[0..5];
    let world = &s[6..11];
}

// 4.3 The Slice Type
fn main() {
    let mut s = String::from("Andy Hoskins");

    let word = first_word(&s);
    println!("{word}")
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); //Returns a byte slice of this String’s contents.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
