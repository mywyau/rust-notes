#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn pattern_match_example() {
    let config_max = Some(0);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}

// In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

fn if_let_example() {  // using if let means les boiler-plate, less reading and writing, less indentation etc.
    let config_max = Some(0);
    if let Some(max) = config_max {  // needs a pattern to match on the lhs
        println!("The maximum is configured to be {}", max);   // lose the enforced exhaustive match
    }
}

fn if_let_else_example(coin: Coin) {
    let mut count = 0;
    println!("counter start value: {}", count);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
        println!("counter: {}", count);
    }
}

// If you have a situation in which your program has logic that is too verbose to express using a match, remember that if let is in your Rust toolbox as well.
fn main() {
    if_let_else_example(Coin::Quarter(UsState::Alabama));
    if_let_else_example(Coin::Penny);
}
