mod front_of_house;

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house;

use back_of_house::Breakfast;

pub fn eat_breakfast_at_restaurant(){
    let mut meal = Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
}

pub fn eat_at_hotel() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// use front_of_house::hosting::*;
// use crate::front_of_house::hosting::*;
use self::front_of_house::hosting::*;

pub fn eat_at_home() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
    seat_at_table();
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

fn eat_at_room() {
    add_to_waitlist();
}

use rand::{Rng, CryptoRng, ErrorKind::Transient};

// use std::io::{self, Write};
use std::io::*;