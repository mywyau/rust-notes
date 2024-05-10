#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Each struct is allowed to have multiple impl blocks. For example, Listing 5-15 is equivalent to the code shown in Listing 5-16, which has each method in its own impl block.
// There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax.
// We’ll see a case in which multiple impl blocks are useful for generic types and traits.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Associated_functions
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle2 =
        Rectangle {
            width: 10,
            height: 40,
        };

    let rectangle3 =
        Rectangle {
            width: 60,
            height: 45,
        };

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle1.area()
    );

    println!("Can rectange1 hold rectangle2? {}", rectangle1.can_hold(&rectangle2));
    println!("Can rectange1 hold rectangle3? {}", rectangle1.can_hold(&rectangle3));

    // Associated functions example
    let square = Rectangle::square(3);

    println!(
        "The area of the square is {} square pixels.",
        square.area()
    );
}


