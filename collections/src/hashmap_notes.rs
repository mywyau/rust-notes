use std::collections::HashMap;

pub fn notes() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_names = String::from("Blue");
    let score: i32 = scores.get(&team_names).copied().unwrap_or(0);

    println!("{}", score)
}


pub fn notes_2() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_names = String::from("Blue");
    let score: i32 = scores.get(&team_names).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key} {value}")
    }
}

pub fn ownership() {

    //TODO: not sure why but it doesn't blow up which the guide suggests it should

    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

pub fn update_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);  // inserting again replaces the key value with the new value.
    // This code will print {"Blue": 25}. The original value of 10 has been overwritten.

    println!("{:?}", scores)
}


