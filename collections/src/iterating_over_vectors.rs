pub fn notes() {
    let v: Vec<i32> = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }
}


pub fn notes_2() {
    let mut v: Vec<i32> = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v)
}