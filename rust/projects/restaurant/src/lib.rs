// nested paths with self-reference
use std::io::{self, Write}

// the glob(al) operator
use std::collections::*; // Be careful with global!

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

    }
}

pub use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house;
    pub fn eat_at_restaurant () {
        //relative path
        hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast
        let mut meal = crate::back_of_house::Breakfast::summer("Rye");
        // Change our mind aboutr what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it;
        // we're not allowed to see or modify the seasonal fruit
        // that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
        
        let order1 = crate::back_of_house::Appetizer::Soup;
        let order2 = crate::back_of_house::Appetizer::Salad;
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup, 
        Salad,
    }
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    //--snip--
}

fn function2() -> IoResult {
    // --snip--
}


