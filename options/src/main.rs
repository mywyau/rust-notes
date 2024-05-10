fn options() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;


    let is_it_an_option = some_number.is_some();
}


fn main() {
    let some_five = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let is_it_an_option = some_five.is_some();
    let mut using_map = some_five.map(|f| f + 1);


    println!("Is the value [some_number] and option? {}", is_it_an_option);
    println!("Adding 1 to [some_five] = {}", using_map.get_or_insert(0));
}
