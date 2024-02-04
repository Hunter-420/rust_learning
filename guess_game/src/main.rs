/*
 * To obtained user input and  then print the result as output, we need to bring io input/output
 * library into scope
 * The io library comes from the standard library, known as std 
 */
use std::io;
use rand::Rng;
use std::cmp::Ordering;
/*
 * By default, Rust has a set of items defined in std library that it brings into the scope of
 * every program. This set is called prelude.
 *
 * If a type we want is not in prelude then we need to bring that into scope explicitly with
 * a use statement. As we do above io module isn't  in prelude so we bring io module which is
 * inside std library(top-level module) which have an ability to accept user input. 
 */

/*
 * fn syntax declares a new function, the parentheses, () indicates there are no parameters and
 * curly bracket, { starts the body of the function
 * and
 * the main function is the entry point into the program 
 */

fn main() {
    // println! is a macro that prints a string to thr screen
    println!("Guess the number: ");

    // we call the rand::thread_rng function that givsz us a particular random number generator
    // we're going to use one that is local to the current thread execution and is seeded by the
    // operating system. Then we call the gen_range method on the random number generator. This
    // method is defined by Rng trait that we brought into scope with the use rand::Rng; statement.
    // The gen_range method takes a range expression as an argument and generates a random number
    // in range. The kind of range expression we are using here takes the form start..==end and is
    // inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number
    // between 1 and 100 
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess: ");
    
    // the code is printing a prompt starting with what the game is and requesting input from the
    // user
    
    //Storing values in variable
    //lets create a mutable variable that is currently bound to a new, empty instace of a String
    let mut guess = String::new();
    // in rust, variables are immutable by default which means once we give the variable vakue, the
    // value won't change. TO make a variable mutable, we add mut before variable name
    
    /* -- Receiving user input 
      we include the io functionality from the standard library with use std::io on the first line
      of the program. Now we will call the stdin function from io module, which will allow us to
      handle user input
    */

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
 
    // trim method on a String instance will eliminate any whitesspace at the beginning and end 
    let guess: u32 = guess.trim().parse().expect("Please tye a number!");
    // comparing the huess to the secret number 
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
    
}

