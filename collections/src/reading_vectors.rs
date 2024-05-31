pub fn notes() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // let does_not_exist: &i32 = &v[100];  // causes program to panic
    let does_not_exist_v2: Option<&i32> = v.get(100);  // does not panic and will return None which we can handle.

    match does_not_exist_v2 {
        Some(element) => println!("trying to get the 100 index: {element}"),
        None => println!("Index out of bounds"),
    }
}

// The code  might look like it should work: why should a reference to the first element care about changes at the end of the vector?
// This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the
// vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements
// next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory.
// The borrowing rules prevent programs from ending up in that situation.

pub fn notes_2() {

    // this will not compile

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let first: &i32 = &v[0];  // holding an immutable reference

    // v.push(6);  // trying to mutate the initial vector, this code is invalid, mutable borrow

    println!("The first element is: {first}");
}