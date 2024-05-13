use std::fmt;
use std::io;

// fn function1() -> fmt::Result {  // when we have 2 of the same name of e.g. a type or method we explicitly call the parent (where it comes from). Rust will not compile otherwise
// also helps us out as devs

//     // --snip--
// }
//
// fn function2() -> io::Result<()> {
//     // --snip--
// }

// Aliases using the `as` keyword
// we can also fix this issue using import aliases,

// use std::fmt::Result;
// use std::io::Result as IoResult;
//
// fn function1() -> Result {
//     // --snip--
// }
//
// fn function2() -> IoResult<()> {
//     // --snip--
// }

fn main() {
    println!("Hello, world!");
}
