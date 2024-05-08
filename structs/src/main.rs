// Using Structs to Structure Related Data

// A struct, or structure, is a custom data type that lets you package together and name multiple related values
// that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.
// In this chapter, we’ll compare and contrast tuples with structs to build on what you already know and demonstrate when structs are a better way to group data.

// We’ll demonstrate how to define and instantiate structs. We’ll discuss how to define associated functions, especially
// the kind of associated functions called methods, to specify behavior associated with a struct type. Structs and enums (discussed in Chapter 6)
// are the building blocks for creating new types in your program’s domain to take full advantage of Rust’s compile-time type checking.


fn main() {
    println!("{}", instantiate_struct().email);
}

struct User {   // similar to a class/case class but need an instance to be useful
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn instantiate_struct() -> User {
    let mut user1 =
        User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

    user1.email = String::from("another_email@example.com");  // we can call values from a struct by using dot notation. If it's mutable then we can change the field value
    // Note the entire instance must be mutable, Rust doesn't allow us to mark only certain fields as mutable.
    // Like Scala case classes we can return the entire instance/updated instance to retrieve a new instance of the Struct.
    user1
}

fn struct_init(email: String, username: String) -> User {
    User {
        active: true,
        username,  // since we want to use defaults we can just use the init shorthand syntax sugar
        email,  // since we want to use defaults we can just use the init shorthand syntax sugar
        sign_in_count: 1,
    }
}

fn struct_update_syntax(email: String, username: String) -> User {
    let mut user1 =
        User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };


    let user2 =
        User {
            active: user1.active,
            username: user1.username, // similar to copying case classes and updating them based on previous data structures we can update the Struct using user1 values for user2
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };

    let user3 =
        User {
            email: String::from("something_crazy@example.com"),
            ..user2  // we can copy data easily using shorthand update syntax.   // however we cannot reference user1, since we used user1 data for user2
            // user2 update syntax must come last.
        };

    user3

    // Note that the struct update syntax uses = like an assignment; this is because it moves the data, just as we saw in the “Variables and Data Interacting with Move” section.
    // In this example, we can no longer use user1 as a whole after creating user2 because the String in the username field of user1 was moved into user2.
    // If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1,
    // then user1 would still be valid after creating user2. Both active and sign_in_count are types that implement the Copy trait,
    // so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.
}

fn tuple_structs() {

    // Rust also supports structs that look similar to tuples, called tuple structs.
    // Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather,
    // they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple
    // a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Note that the black and origin values are different types because they’re instances of different tuple structs.
    // Each struct you define is its own type, even though the fields within the struct might have the same types.
    // For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values.
    // Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by
    // the index to access an individual value.
}

struct AlwaysEqual;

fn unit_type_struct(){
    let subject = AlwaysEqual;
}



























