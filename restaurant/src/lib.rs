mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;

fn deliver_order() {}

fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I would like to have {} toast, please", meal.toast);

    hosting::add_to_waitlist();
}
