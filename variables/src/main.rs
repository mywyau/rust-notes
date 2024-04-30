fn main() {

    // this block of code is invalid, variables by default are immutable
    let x = 5;
    println!("The value of x is {x}");

    // x = 6;
    println!("The value of x is now: {x}");

    // we need to make the let variable mutable
    // mutability is fine on a small scale and with few mutations, over time and many changes it gets a little un-wieldly
    // mutability really adds convenience and often times performance

    let mut y = 5;

    println!("The value of y is {y}");

    y = 9999;
    println!("The value of y is now:{y}");

    // Constants

    // we are not allowed to use mut with Constants

    // always immutable, and can be in global scope

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;   // constants naming convention in Rust is uppercase_snake_case

    // The compiler is able to evaluate a limited set of operations at compile time, which lets us choose
    // to write out this value in a way that’s easier to understand and verify, rather than setting this
    // constant to the value 10,800. See the Rust Reference’s section on constant evaluation for
    // more information on what operations can be used when declaring constants.

    // Shadowing

    let shadow = 5;
    let shadow = shadow + 1;

    {
        let shadow = shadow * 2;
        println!("The value of shadow in the inner scope is: {shadow}");
    }

    println!("The value of x shadow the outer scope is: {shadow}");

    let mut spaces = "    ";
    // spaces = spaces.len(): we cannot mutate a mutable references type via shadowing.

    // Data Types

    // let guess = "42".parse().expect("Not a number!");

    // Integer types

    //           | signed     | unsigned
    //  8-bit    | i8         | unsigned
    //  16-bit   | i16        | unsigned
    //  32-bit   | i32        | unsigned
    //  64-bit   | i64        | unsigned
    //  128-bit  | i128       | unsigned
    //  arch     | isize      | usize

    // signed and unsigned refer to if the type possibly can be negative.

    // if positive, then no sign
    // signed numbers are stored using two' complement representation.

    // Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses
    // isize/usize is also determined on the architecture of your computer system.

    // unsure which to use? well start with default i32, when indexing collections for example then use isize or usize

    // 0-255  is u8 if you try to use a number outside this range, an integer overflow will occur.
    // if this overflow occurs then program goes into 'panic' mode during runtime.

    /*
        When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics.
        Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold
        “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on.
        The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have.
        Relying on integer overflow’s wrapping behavior is considered an error.

        To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

        Wrap in all modes with the wrapping_* methods, such as wrapping_add.
        Return the None value if there is overflow with the checked_* methods.
        Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
        Saturate at the value’s minimum or maximum values with the saturating_* methods.
    */
}
