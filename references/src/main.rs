fn main() {
    // print_length();
    // cannot_modify_borrowed_stuff();   // does not compile will give you error about modifying a borrowed value.

    // can_modify_mutable_references();
    // mix_mutable_with_immutable_invalid();
    mix_mutable_with_immutable_valid();

    // let reference_to_nothing = dangling_reference();   // reference is dropped, but compiler catches this
    let reference_to_nothing = no_dangling_reference();   // compiles since we do no have a dangling reference.
}

// to avoid the whole tupling and return value function situation in the ownership package regarding variables is to use a 'reference'
// to denote a 'reference' you use the & operator in front of a function parameter and variable reference.

fn calculate_length(s: &String) -> usize {  // s type signature is annotated with & which tells us and Rust it is a reference, helping out with ownership.
    s.len()
}

// The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
// Because it does not own it, the value it points to will not be dropped when the reference stops being used.
fn print_length() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);   // s1 is annotated with & which tells us and Rust it is a reference, helping out with ownership

    println!("The length of '{}' is {}.", s1, len)
}

// the opposite of referencing is called dereferencing. De-referencing's operator is *

// when creating a reference we call this concept borrowing. Similar to real life if someone owns something, you can borrow it from them. When you are done you give it back to them.
// You don't own it.

fn cannot_modify_borrowed_stuff() {
    let s = String::from("hello");
    change(&s);  // we are trying to modify the reference. change function takes reference not a value, it then tries to modify it which obviously fails.
}

fn change(some_string: &String) {
    // some_string.push_str(", world");  // trying to modify the String, but since it is borrowed, this is not allowed.
}

// we can however modify a mutable reference.

fn change_v2(some_string: &mut String) {
    some_string.push_str(", world");
}

fn can_modify_mutable_references() {
    let mut s = String::from("hello");
    change_v2(&mut s);  // we are trying to modify the reference. change function takes reference not a value, it then tries to modify it which obviously fails.
}

// mutable references have a big restriction
// if you have a mutable reference to a value, you can have no other references to that value.

// fn multiple_mutable_references() {   //  this is not allowed, will not compile
//
//     let mut s = String::from("hello");
//     let r1  = &mut s;
//     let r2  = &mut s;
//
//     println!("{}, {}", r1, r2);l
//
// }

// to fix the above issue of simultaneous mutable references we can use scope

fn scoped_references() {
    let mut s = String::from("Hello");

    {
        let r1 = &mut s;
    }  // r1 goes out of scope here so we can now make another reference within the same function to s

    let r2 = &mut s;
}

// rust also enforces a similar rule when mixing mutable and immutable references.

fn mix_mutable_with_immutable_invalid() {
    let s = String::from("Hello");
    let r1 = &s;        // these are fine since immutable
    let r2 = &s;        // these are fine since immutable
    let r3 = &s;        // these are fine since immutable
    // let r3 = &mut s;   // not allowed   will get error:        ^^^^^^ cannot borrow as mutable

    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}, and {}", r1, r2, r3);
}

fn mix_mutable_with_immutable_valid() {
    let mut s = String::from("Hello");  // initally declared as mutable
    let r1 = &s;        // these are fine since immutable
    let r2 = &s;        // these are fine since immutable
    println!("immutable r1: {}, r2: {}", r1, r2);

    let r3 = &mut s;        // this is allowed since out of scope of the other references. and initial reference is declared mutable
    println!("mutable r3:{}", r3);
}


// Dangling references

// In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in
// memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.
// In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data,
// the compiler will ensure that the data will not go out of scope before the reference to the data does.

// fn dangling_reference() -> &String {  // error will mention lifetime parameters.  // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// // Danger!

fn no_dangling_reference() -> String {
    let s = String::from("hello");
    s
}






