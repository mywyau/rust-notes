fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Using Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn derived_traits() {}

fn main() {

    let width1 = 30;
    let height1 = 50;

    let rectangle1 =
        Rectangle {
            width: width1,
            height: height1,
        };

    let scale = 2;
    let rectangle2 =
        Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

    println!("[area] The area of the rectangle is {} square pixels.", area(width1, height1));
    println!("[area_tuples] The area of the rectangle is {} square pixels.", area_tuples((width1, height1)));
    println!("[area_structs] The area of the rectangle is {} square pixels.", area_structs(&rectangle1));

    println!("rect1 is {:?}", rectangle1);  // can use {:#?} for pretty printing

    dbg!(&rectangle2);
    // println!("rect1 is {}", rect1); // this naturally does not work
}