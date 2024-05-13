use crate::front_of_house::hosting;

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

pub fn eat_at_restaurant() {

    // using the use keyword
    hosting::add_to_waitlist();   // easier syntax, still checks privacy

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

mod customer {

    use crate::front_of_house::hosting; // required or if commented out the super works too  (1)

    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist();  // this does not work it either requires a use within the mod or using super
        hosting::add_to_waitlist();    // add_to_waitlist() comes from hosting, kinda like when you have two methods the same but from diff libraries in Scala.
        // Helps identify the difference. between similarly named functions.
        // we use a slightly longer path to specify add_to_waitlist() is somewhere else and where it is located. It is a balance between convenience/readability and clarity on locality of code.

        // super::hosting::add_to_waitlist();    need to comment out (1), then uncomment to see it able to pull the correct path from the parent of mod customer at top of file.
    }
}


