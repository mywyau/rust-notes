fn main() {
    // print_length();
    // cannot_modify_borrowed_stuff();   // does not compile will give you error about modifying a borrowed value.

    // can_modify_mutable_references();
    // mix_mutable_with_immutable_invalid();
    // mix_mutable_with_immutable_valid();

    // let reference_to_nothing = dangling_reference();   // reference is dropped, but compiler catches this
    let reference_to_nothing = no_dangling_reference();   // compiles since we do no have a dangling reference.

    // Slices

    // This program compiles without any errors and would also do so if we used word after calling s.clear().
    // Because word isn’t connected to the state of s at all, word still contains the value 5. We could use that value 5
    // with the variable s to try to extract the first word out, but this would be a bug because the contents of s have changed since we saved 5 in word.

    // Having to worry about the index in word getting out of sync with the data in s is tedious and error prone!

    let mut s = String::from("hello world");

    // let word = first_word(&s); // compiles and works but not safe, will return the value 5 however the .clear method below clears the word. The string and it's length is now ouw out of sync

    // let word = first_word_rewrite(&s); // safer, will throw a warning
    //
    // s.clear(); // error!
    //
    // println!("the first word is: {}", word);
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

// Slices

fn slices() {
    let s = String::from("hello");
    let hello = &s[0..5];
    let world = &s[6..11];
}

fn slices_syntax_sugars_head() {
    let s = String::from("hello");
    let slice = &s[0..2];       // these are equivalent
    let slice = &s[..2];        // these are equivalent
}

fn slices_syntax_sugars_last() {
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];       // these are equivalent
    let slice = &s[3..];        // these are equivalent
}

fn slices_syntax_sugars_entire() {
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];       // these are equivalent
    let slice = &s[..];        // these are equivalent
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// why use Slices, to build more robust programs. It's to ensure the references of Strings going into functions remain valid.

// We now have a straightforward API that’s much harder to mess up because the compiler will ensure the references
// into the String remain valid. Remember the bug in the program in Listing 4-8, when we got the index to the end
// of the first word but then cleared the string so our index was invalid? That code was logically incorrect but
// didn’t show any immediate errors. The problems would show up later if we kept trying to use the first word index with an emptied string.
// Slices make this bug impossible and let us know we have a problem with our code much sooner. Using the slice version of first_word will throw a compile-time error:


fn first_word_rewrite(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


// String Literals as Slices

fn string_literals_slices() {
    let s = "Hello World!";

    // fn first_word(s: &String) -> &str {}  // possible signature but there is a better one.

    // A more experienced Rustacean would write the signature shown instead because it allows us to use the same function on both &String values and &str values.

    // If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String.
    // This flexibility takes advantage of deref coercions

    // fn first_word(s: &str) -> &str {};
}

fn program_string_literal_showcase() {
    let my_string = String::from("Hello world!");
    let word = first_word_rewrite(&my_string[0..6]);
    let word = first_word_rewrite(&my_string[..]);

    let word = first_word_rewrite(&my_string);

    let my_string_literal = "hello world";
    let word = first_word_rewrite(&my_string[0..6]);

    // Because string literals are string slices already,
    // this works too, without the slice syntax!
    let word = first_word_rewrite(my_string_literal);
}

fn other_slices() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}











