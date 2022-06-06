
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, log};
use std::collections::HashMap;

mod info;

// A constant list of food choices that the restaurant offers. 
// The index of food in the MENU_ITEMS array matches its respective price index in the MENU_PRICES
const MENU_ITEMS: [&str;16] = ["Striploin steak", "T-Bone steak", "Boz Centre Cut Fillet", "Tomahawk Ribeye steak", "Fried egg", "Prawns", "Sauteed mushroom", "Onion rings", "Roast baby potatoes", "Sweet potato fries ", "Roast veggies", "Tender greens salad ", "Chocolate Chip Cookie", "Chocolate Cake", "Traditional Apple Pie", "Selection of Ice-cream"];
const MENU_PRICES: [f32; 16] = [40.0, 37.0, 35.0, 75.0, 3.0, 5.0, 4.0, 3.50, 7.0, 6.50, 7.0, 6.50, 8.0, 8.0, 8.50, 2.0, ];

/* 
    This is a restaurant service contract where clients can get to a restaurant and get the restaurant's
    near account. With it, they can check the menu, make food orders, add ratings and also pay for their food items.
    This can all be done securely and anoymously.
*/
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // HashMap is used to store multiple sessions of different clients on different unique tables.
    // Table numbers are therefore used as keys.
    table_allocation: HashMap<u8, Client>,
    // all_ratings stores all ratings made by clients and the avg_rating stored the final computed average rating
    all_ratings: Vec<u8>,
    avg_rating: f32,
}

/* 
    This is all the clients information we'll be storing per table in the contract.table_allocation,
    that is the food the client requested, the table and also the cost of the food in NEAR
*/
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
#[derive(Clone)]
pub struct Client {
    // SETUP CONTRACT STATE
    table: u8,
    food: String,
    cost: f32,
}

#[near_bindgen]
impl Contract {
    // call to get initial instructions on how to use contract
    pub fn hello(&self) {
        info::hello(self.avg_rating);
    }
    
    // Call to get the menu
    pub fn menu() {
        info::menu();
    }

    // Initialization function for the contract and setting the initial avg_rating
    #[result_serializer(borsh)]
    #[init]
    pub fn new() -> Contract {
        Contract {
            table_allocation: HashMap::new(),
            all_ratings: Vec::new(),
            avg_rating: 5.0,
        }
    }
    /*
        The main function which the client calls to make an meal order. The client passes his food choice and table number.
        The data provided is used to initialized a new instance of a client object only if the food provided exists in the MENU_ITEMS.
    */
    pub fn main(&mut self, table_number: u8, food_choice: String) {
        // convert food_choice from string to str so that it can be used to iterate through MENU_ITEMS
        let food: &str = &*food_choice;
        log!("Table number {} your order is {} ", &table_number, &food);
        // Check if the client's food exists in the MENU_ITEMS
        if MENU_ITEMS.contains(&food) {
            env::log_str("Your order is confirmed");
        // Get the index of the food from the MENU_ITEMS so as to match the order's price
            let food_index: usize = MENU_ITEMS.iter().position(|&x| x == &food_choice).unwrap();
            // The cost is currently in dollars thus divided by the current value of NEAR to convert it to near
            let cost: f32 = MENU_PRICES[food_index] / 5.64; 
            let client_new = Client {
                    table : table_number,
                    food: food_choice,
                    cost : cost,
                };
            // call the add_client method passing the generated Client and the table_number as key
            self.add_client(client_new.clone(), table_number);
            log!("Your food should cost: {} near", self.table_allocation[&table_number].cost);
        }else {
            env::log_str("Your does not exist in our inventory, please try again");
            
        };    
    }
    
// non public function to add the newly generated client to the table_allocation hashmap
    #[result_serializer(borsh)]
    fn add_client(&mut self, client: Client, table: u8) {
        self.table_allocation.insert(table, client);
    }
// Called to get the reciept for the cost of meals per table_number
    pub fn reciept(&self, table_number: u8) {
        log!("You will be charged {} near", self.table_allocation[&table_number].cost);
    }

// A payable function where clients can pay for their meals using NEAR tokens
    #[payable]
    pub fn pay(&mut self, table_number: u8) {
        // Assign attached near and the cost of food for the table to variables
        let tokens = env::attached_deposit();
        let charge = self.table_allocation[&table_number].cost;
        log!("deposited {} ", tokens );
        // convert unsigned integer to float and yocto to near 
        let token_float = ((tokens as f32)/1000_000_000_000_000_000_000_000.0) + 0.0001;
        log!("token float: {}, cost: {}", token_float, charge);
        // if checks to compare the token recieved to the expected charge for the meals
        if token_float == charge {
            env::log_str("successful");
            return;

        } if token_float > charge {
            log!("You paid more by {} we hope it's a tip", (charge)-token_float);
            return;

        } if token_float < charge {
            log!("You paid less by {} please consider paying up", (charge)-token_float);
            return;

        }else {env::log_str("unsuccessful please add an amount")}
    }
// Manage client ratings and Restaurant avarage ratings
    pub fn ratings(&mut self, rating: u8, table_number: u8) {
        // Check if the table number making the request is a valid client
        if !self.table_allocation.contains_key(&table_number) {
            log!("sorry only clients can rate our services");
            return
        }
        // Add clients ratings to the all ratings vector
        self.all_ratings.push(rating);
        // Compute the average ratings of the restaurant
        let ratings_count = self.all_ratings.len() as f32;
        // Get a sum of total ratings inclusive of the newly added rating using a for loop
        let mut total_ratings = 0.0;

        for rate in &self.all_ratings {
            total_ratings += *rate as f32;
        }
        // Compute a new average rating and Update it to the Restaurant struct
        self.avg_rating = total_ratings / ratings_count;
        log!("Current restaurant ratings stand at {}", self.avg_rating)
    }
//  a test functions
    pub fn views(&self) {
        let num_of_clients = self.table_allocation.len();
        log!("You currently have {} clients", num_of_clients);
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
