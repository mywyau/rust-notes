fn main() {

    // statements do not return values
    let statement = 6;

    another_function();
    another_function_parameters(5);
    print_labeled_measurement(10, "h");

    statement();
    expression();

    let x = five();
    println!("The value of x is {x}")
}

fn another_function() {
    println!("Another function");
}

fn another_function_parameters(x: i32) {
    println!("Another function with parameters: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn statement(){
    let z = 8;
}

fn expression(){
    let y = {
        let x = 3;
        x + 1  // equals 4 so y can bind to the value 4
    };
    println!("The value of y is {y}");
}

fn five() -> i32 {  // -> is the return type so in this case return type is i32
    5
}

// fn plus_one(x: i32) -> i32 {
//     x + 1;  this is  an invalid return type
// }





