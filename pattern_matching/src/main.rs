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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {   // multi-lines like Scala
            println!("Lucky Penny");  // adding a side-effect here
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {   //  by adding the state variable to Quarter we can extract it or manipulate it, adding richness to the type.
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn pattern_match_game() {
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}

    fn move_player(num_spaces: u8) {}

    fn reroll() {}

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other)   // catch-all value
        // _ => reroll()  // if we wanted to not use the catch-all value
        _ => ()   // unit type do nothing
    }
}


fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let alaska_quarter = Coin::Quarter(UsState::Alaska);

    println!("The value of the coin is: {}", value_in_cents(penny));
    println!("The value of the coin is: {}", value_in_cents(nickel));
    println!("The value of the coin is: {}", value_in_cents(dime));
    println!("The value of the coin is: {}", value_in_cents(quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
