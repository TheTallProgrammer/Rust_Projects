// io input/output library into scope. The io library comes from the standard library, known as std:
use std::io; 
use std::cmp::Ordering;
use rand::Rng;

// The fn syntax declares a new function, the parentheses, (), indicate there are no parameters, and the curly bracket, {, starts the body of the function
fn main() {

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {

        println!("Please input your guess.");

        // let apples = 5; // immutable (value can't change)
        // let mut bananas = 5; // mutable (value can change)
        let mut guess = String::new();

        // Allows for user input
        io::stdin()

            // .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in.
            // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
            .read_line(&mut guess)

            // This is basically catch, displays this method if there is an error involving .read_line
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // The {} set of curly brackets is a placeholder: think of {} as little crab pincers that hold a value in place. You can print more than one value using curly brackets: the first set of curly brackets holds the first value listed after the format string, the second set holds the second value, and so on. Printing multiple values in one call to println!
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Breaks out of the loop if the correct number is found
            }
        }
    }


}