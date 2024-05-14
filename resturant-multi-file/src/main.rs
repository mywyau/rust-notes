mod front_of_house;

fn main() {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }

    println!("this is within main");
    eat_at_restaurant();
}
