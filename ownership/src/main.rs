fn main() {
    // string_ownership_example();
    // rust_ownership_example()
    // ownership_and_functions()
    // return_values_and_scope()
    tedious_variables_regarding_ownership()
}

fn string_ownership_example() {
    let mut s = String::from("hello");
    // do some stuff with s

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s);
}   // afterwards the string s is no longer valid, freeing up memory. Rust will call the drop function to help clean things up.

// This ownership pattern feels trivial at the moment but will heavily influence how Rust code is written.

// double-free error is a memory safety bug when 2 references pointing to the same bit of memory goes out of scope and is dropped.


fn free_memory_error() {
    /*
        Notes on why this is important.
        This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory.
        This is known as a double free error and is one of the memory safety bugs we mentioned previously.
        Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    */


    let s1 = String::from("hello");
    let s2 = s1;
    // both s1 and s2 point to the same piece of memory stored on the stack when this goes out of scope both s1 and s2 would try to drop
    // the memory storing the String value. This is what would happen if Rust did not handle things they way it does.

    println!("{}, world!", s2);  // this is valid since s1 is no longer valid once s2 references it.
}

fn rust_ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);  // try uncommenting and running this, looks valid in IDE but when compiled this bombs out
    println!("{}, world!", s2);  // s2 is the correct reference. Rust already forgot about s1 to prevent free_memory errors
}

/*
    If you’ve heard the terms shallow copy and deep copy while working with other languages,
    the concept of copying the pointer, length, and capacity without copying the data probably
    sounds like making a shallow copy. But because Rust also invalidates the first variable,
    instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2.

    In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data.
    Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
*/

fn clone_example() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();   // we explicitly copy the heap data here so s1 is also valid and Rust will not drop/clean s1 up.

    println!("s1 = {}, s2 = {}", s1, s2);

    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive.
    // It’s a visual indicator that something different is going on.
}

fn stack_only_copy_example() {
    let x = 5;
    let y = x;  // certain types do not get dropped by Rust. Integers are one of them there is no need.

    println!("x = {}, y = {}", x, y);

    // If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied,
    // making them still valid after assignment to another variable.
    // Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait.
}

// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating-point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.


fn ownership_and_functions() {
    let s = String::from("hello");

    takes_ownership(s); // s takes_ownership has taken ownership of s, we cannot use it anymore after this point.
    let x = 5;
    // println!("{} world", s);   // try to uncomment this s

    makes_copy(x);   // x goes into the make_copy function, i.e  make_copy has taken ownership, but since x was in i32 it's still fine to use afterwards
    // println!("[ownership_and_functions] we can still use {x} but not the String value {s}'");   // not allowed
    println!("[ownership_and_functions] we can still use {x} but not the String value 's'")
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn return_values_and_scope() {
    let s1 = gives_ownership();  // this example does nothing but move the return value into the let variable stored here
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // println!("{s2}");  // this println! is not allowed to be used since it is gone into the function takes_and_gives_back(s2)
    // but since takes _and_gives_back is an identity function it returns the value s2, doing nothing.
    println!("{s3}")  // instead we can use s3 since it stores s2.
}
// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // this value is store in s1 within return_values_and_scope()
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();  // len() returns the length of a string
    (s, length)
}

fn tedious_variables_regarding_ownership() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}