mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,   // not public hence we cannot modify seasonal_fruit
    }

    impl Breakfast {   // basically like a instantiated Breakfast with a summer function which sets the toast and season_fruit. we can update/modify toast but not the fruit
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // tbh this is pointless since enums are public by default, this is because their values are not useful if they were not public,
    // on the otherhand structs are private by default as there is value to having some fields being private when organising code/data.
    pub enum Appetizer {   // entire enum is public hence we are allowed to use the Soup and Salad enum outside of the `mod back_of_house`
        Soup,
        Salad
    }

}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");   // a call to the summer breakfast.
    meal.toast = String::from("Wheat");   // allowed to modify toast since it is public
    // however we cannot modify seasonal_fruit since it is private
    println!("I'd like {} toast please", meal.toast)

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant_2() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}