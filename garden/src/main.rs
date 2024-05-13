use crate::garden::vegetables::Asparagus;

pub mod garden;   // points to garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant)
}
