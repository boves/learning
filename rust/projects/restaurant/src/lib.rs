mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

    }
}

pub fn eat_at_restaurant () {
    //Absolute path
    crate::front_of_house::hosting:add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}
