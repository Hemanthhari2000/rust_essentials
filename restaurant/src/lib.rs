mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("mango")
            }
        }
    }
}


fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("rice");
    meal.toast = String::from("new_toast");
    println!("I'd like {} please", meal.toast);
}
