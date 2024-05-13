mod front_of_house {
    // rust modules are private by default hiding implementation details. This mean we know what we are allowed to change without breaking outer code.
    pub mod hosting {    // adding pub to mods only allows us to reference the mod but not it's contents. we need to go deeper and make it's content pub to access it

        pub fn add_to_waitlist() {}

        fn add_seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_resturant() {

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


// we will assume this code structure is desirable and correct hence using super will help when we decide to move deliver_order() and the mod back_of_house together.
fn deliver_order() {}   // this is in crate i.e root so we can reference it using super

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();   // mod back_of_house is in the root/crate file so we can call deliver_order using 'super' keyword
    }

    fn cook_order() {}
}


