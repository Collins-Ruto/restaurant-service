use near_sdk::{env, log};
use std::collections::HashMap;

pub fn menu_items() -> HashMap<String, f32> {
    HashMap::from([
    ("Strip steak".to_string(),12.0),
    (stg("T-Bone steak"),17.0),
    (stg("Cut Fillet"),15.0),
    (stg("Ribeye steak"),15.0),
    (stg("Fried egg"),3.0),
    (stg("Prawns"),5.0),
    (stg("Mushroom"),4.0),
    (stg("Onion rings"),3.50),
    (stg("Roast potatoes"), 7.0),
    (stg("Roast veggies"), 8.0),
    (stg("Salad "), 7.65),
    (stg("Apple Pie"), 0.1),
    (stg("Ice-cream"), 6.2)
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
                Striploin steak             $40.0, 
                T-Bone steak                $37.0, 
                Centre Cut Fillet           $35.0, 
                Tomahawk Ribeye steak       $75.0, 
            TOPPINGS
                Fried egg                   $3.0, 
                Prawns                      $5.0, 
                Sauteed mushrooms           $4.0, 
                Onion rings                 $3.50, 
            SIDES       
                Roast baby potatoes         $7.0, 
                Sweet potato fries          $6.50, 
                Roast veggies               $7.0, 
                Tender greens salad         $6.50, 
            DESSERTS
                Baked Chocolate Chip Cookie $8.0, 
                Chocolate Cake              $8.0, 
                Traditional Apple Pie       $8.50, 
                ,Selection of Ice-cream      $2.0,
            Call 
        ")
}