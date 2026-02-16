use rand::Rng; //Rng trait defines methods that random number generators implement
use std::cmp::Ordering; //Ordering is another enum with varients (Less, Greater & Equal)
use std::io; //Use the standard libary (input/output)

fn main() {
    println!("Guess the number!"); //println! is a macro

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");
    loop {
        println!("Please enter a guess");

        let mut guess = String::new(); //a function that returns a new instance of string.

        io::stdin()
            .read_line(&mut guess) //read_line returns a result(enum) result variants are ok & Err
            .expect("Failed to read line");

        //parse method converts a string to a number (u32)
        let guess: u32 = match guess.trim().parse() {
            //Match returns a result enum of ok or Err.
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            //cmp method compares two values
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Winner!");
                break;
            }
        }
    }
}
