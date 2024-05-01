fn main() {
    // less_than_five(7);
    // else_if_example(6);
    // if_and_lets()
    // loop_example();
    // return_value_from_loop();
    // loop_labels()
    // while_loops()
    // for_loop_example();
    rev_example();
}

fn less_than_five(number: i32) {
    if number < 5 {
        println!("condition was true") // we call these blocks of code "arms" of an if expression or match expression
    } else {
        println!("condition was false")
    }
}

fn else_if_example(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}

fn if_and_lets() {
    let condition = true;
    let number =
        if condition {
            5
        } else {
            6
        };

    println!("The value of number is: {number}");
}

fn loop_example() {
    loop {
        println!("again!");   // hit ctrl + c to escape the loop
        // break; use breaks to exit loops
    }
}

fn return_value_from_loop() {
    let mut counter = 0;

    let result =
        loop {
            counter += 1;
            println!("Counter: {counter}");
            if counter == 10 {
                println!("Counter has reached 10, return counter multiplied by 2");
                break counter * 2;  //possible to return values from breaks
            }
        };
    println!("The result is {result}")
}

fn loop_labels() {

    // when dealing with multiple loops you can use loop labels to tell breaks and continues which loops you are referring to.

    let mut count = 0;

    // we will call loop-1 "counting_up"
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                println!("count has reached 2, we break out of loop");
                break 'counting_up;   // we break the "counting_up" loop-1 here
            }
            println!("reduce remaining by 1");
            remaining -= 1
        }
        count += 1
    }
    println!("End count = {count}");
}

fn while_loops() {
    // can loop based on a predicate/condition. Since this is so common instead of if else and breaks encoding this logic most languages include while loops

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("Lift off!!!")
}

fn while_loop_through_array() {

    // this can has problems which can be solved via a for loop

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop_example() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}")
    }
}

fn rev_example() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("Liftoff!!")
}