use std::io;  // import input and out put library

fn main() {     // main function entry point into program

    let apples = 5;  // immutable
    let mut bananas = 7;  // mutable

    // the :: in String::new denotes an associated function. An associated function is a function that's implemented on a type.
    let mut guess = String::new();  //  let keyword, to create a new variable. We add mut to allow for mutation of this variable
    // the new function in this case creates a new empty String which is mutable bound to the variable called guess.

    println!("Please hit enter once to start the game"); // this is the macro version of println notice the !

    let mut emptyString = String::new();
    std::io::stdin()   // this is overkill and not necessary since we already have the import but regardless
        .read_line(&mut emptyString)
        .expect("We have used imports in our code, similar to Scala where we can specify where the method came from");
    // we can perform better error handling but in later chapters

    // this string is the failure message if Result == Err
    // in this case Ok is the number of bytes of the user's input.

    // .expect is error handling on the Result type, it can have 2 states Ok and Err
    // if the Result == Err then the program fails, crashes and burns, if the Result type is Ok then the operation was successful.

    // code will still compile but will throw a warning.
    println!("Guess the number!"); // this is the macro version of println notice the !
    println!("Please input your guess.");

    io::stdin()   // stdin  is for standard input
        .read_line(&mut guess)
        // &mut guess is used to tell the program where to store the user input. In this case it's the mutable empty string.
        // the & in &mut is a reference. it is telling Rust that we want to use guess as a reference. References are immutable by default.
        // Hence we use &mut
        .expect("Failed to read line");

    // string placeholders

    let x = 10;
    let y = 20;

    println!("x:{x}, y: {y}, x + y = {}", {x + y});  // we can use placeholders similar to string interpolation. we can also have empty placeholders

    println!("You guessed: {guess}");
}
