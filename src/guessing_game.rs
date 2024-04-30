// import input and out put library
pub mod guessing_game {
    use std::cmp::Ordering;
    use std::io;

    use rand::Rng;

    pub fn stuff() {
        let apples = 5;  // immutable
        let mut bananas = 7;  // mutable

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

        // string placeholders

        let x = 10;
        let y = 20;

        println!("x:{x}, y: {y}, x + y = {}", { x + y });  // we can use placeholders similar to string interpolation. we can also have empty placeholders
    }

    pub fn run_game() {
        use std::io;
        // println!("Guess the number!"); // this is the macro version of println notice the !

        // let secret_number = rand::thread_rng().gen_range(1..=100);

        // println!("Please input your guess.");

        // the :: in String::new denotes an associated function. An associated function is a function that's implemented on a type.
        let mut guess = String::new();  //  let keyword, to create a new variable. We add mut to allow for mutation of this variable
        // the new function in this case creates a new empty String which is mutable bound to the variable called guess.

        io::stdin()   // stdin  is for standard input
            .read_line(&mut guess)
            // &mut guess is used to tell the program where to store the user input. In this case it's the mutable empty string.
            // the & in &mut is a reference. it is telling Rust that we want to use guess as a reference. References are immutable by default.
            // Therefore, we use &mut
            .expect("Failed to read line");

        // let guess: u32 =   // rust allows shadowing of variable names so we do not need 2 variants e.g. guess_str and guess
        //     guess
        //         .trim()  // time needed to compare to u32
        //         .parse()
        //         .expect("Please type a number!");


//                .expect("Please type a number!");

        // println!("The secret number is : {secret_number}");
        let secret_number = rand::thread_rng().gen_range(1..=100);

        loop {
            println!("Please input your guess.");


            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 =
                match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too big"),
                Ordering::Equal => {
                    println!("You win!");
                    println!("The secret number was: {secret_number}");
                    break;  // adding break statement in loop to quit out of the game
                }
            }
        }
    }
}
