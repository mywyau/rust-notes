pub fn notes() {

    let v: Vec<i32> = Vec::new();

    let v_2: Vec<i32> = vec![1, 2, 3];

    let mut v_mutable = Vec::new();

    v_mutable.push(5);
    v_mutable.push(6);
    v_mutable.push(7);
    v_mutable.push(8);

    println!("{:?}", v_2);

    let mut count = 0;
    for element in v_mutable {
        println!("element {count} = {element}");
        count += 1
    }
}