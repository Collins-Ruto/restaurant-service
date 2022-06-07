use near_sdk::{env, log};
use std::collections::HashMap;

pub fn menu_items() -> HashMap<String, f32> {
    HashMap::from([
    ("strip steak".to_string(),12.0),
    (stg("cut fillet"),15.0),
    (stg("ribeye steak"),15.0),
    (stg("fried egg"),3.0),
    (stg("prawns"),5.0),
    (stg("mushroom"),4.0),
    (stg("roast potatoes"), 7.0),
    (stg("roast veggies"), 8.0),
    (stg("salad "), 7.65),
    (stg("apple Pie"), 8.50),
    (stg("ice-cream"), 6.2)
    ])
}

fn stg(food: &str) -> String {
    String::from(food)
}

pub fn hello(rating: f32) {
    log!("
        Welcome to Gees reastaurant \n
        Our services currently have an average rating of {}
    ", rating);
    env::log_str(
        "Function calls are as follows:
        menu: 'menu'
        order: 'order {\"table_number\", \"food_choice\"}'
        reciept: 'reciept {\"table_number\"}'
        pay: 'pay {\"table_number\"}'
        ratings: 'ratings {\"rating\", \"table_number\"}'
        "
    );
}

pub fn menu() {
    env::log_str("\n
            BEEF
                Strip steak                 12.0 Ⓝ, 
                Cut Fillet                  15.0 Ⓝ, 
                Ribeye steak                15.0 Ⓝ, 
            TOPPINGS
                Fried egg                   3.00 Ⓝ, 
                Prawns                      5.00 Ⓝ, 
                mushrooms                   4.00 Ⓝ, 
            SIDES
                Roast potatoes              7.00 Ⓝ, 
                Roast veggies               8.09 Ⓝ, 
                Salad                       7.65 Ⓝ, 
            DESSERTS
                Apple Pie                   8.50 Ⓝ, 
                Ice-cream                   6.20 Ⓝ,
        ")
 }