fn main() {

    // Booleans

    let t = true;
    let f: bool = false;

    // Chars

    // chars are 4 bytes in size and represents unicode Scalar Value, which means it can represent more than just ASCII.
    // Chinese, Japanese, emojis and zero width spaces are all valid chars.

    let c: char = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';


    // Compound types
    // Tuples

    let tupled: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tupled;   // decompose/destructure tuples

    println!("The value of y is {y}");  // use tuple in placeholders

    let tupled_v2: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tupled_v2.0;

    let six_point_four = tupled_v2.1;

    let one = tupled_v2.2;

    // println!("tupled_v2: {}");   // cannot print out tuple
    println!("five_hundred: {five_hundred}");
    println!("six_point_four: {six_point_four}");
    println!("one: {one}");

    // Arrays

    // Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
    // Arrays are useful when you want your data allocated on the stack rather than the heap

    // Vectors are more flexible than arrays

    // However, arrays are more useful when you know the number of elements will not need to change.
    // For example, if you were using the names of the month in a program, you would probably use an array rather
    // than a vector because you know it will always contain 12 elements:

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let months: [&str; 12] =
        [
            "January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"
        ];

    let a = [3; 5];   // [3,3,3,3,3]    a semi-colon used after an element produces duplicates with the second param denoting quantity

    let b = [1, 2, 3, 4, 5];
    let first = b[0];
    let second = b[1];

    // println!("b: {b}"); unable to print out array
    println!("first element: {first}");
    println!("second element: {second}");
}
