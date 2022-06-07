use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
use std::collections::HashMap;

mod info;
mod payments;

/*
    This is a restaurant service contract where clients can get to a restaurant and get the restaurant's
    near account. With it, they can check the menu, make food orders, add ratings and also pay for their food items.
    This can all be done securely and anoymously.
*/

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // HashMaps are used to store multiple sessions of different clients on different unique tables.
    // A list of food choices that the restaurant offers and prices for each also here
    // Table numbers are therefore used as keys.
    menu: HashMap<String, f32>,
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
#[derive(Default, BorshDeserialize, BorshSerialize, Clone)]
pub struct Client {
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
    // Private thus only called by restaurant owner
    #[result_serializer(borsh)]
    #[init]
    #[private]
    pub fn new() -> Contract {
        Contract {
            menu: info::menu_items(),
            table_allocation: HashMap::new(),
            all_ratings: Vec::new(),
            avg_rating: 5.0,
        }
    }
    /*
        The order function which the client calls to make an meal order. The client passes his food choice and table number.
        The data provided is used to initialized a new instance of a client object only if the food provided exists in the MENU_ITEMS.
    */
    pub fn order(&mut self, table_number: u8, food_choice: String) {
        let food_choice = food_choice.to_lowercase();
        log!("Table number {} your order is {} ", &table_number, &food_choice);
        // Check if the client's food exists in the MENU_ITEMS
        if self.menu.contains_key(&food_choice) {
            env::log_str("Your order is confirmed");
            // // Get the index of the food from the MENU_ITEMS so as to match the order's price
            // let food_index: usize = MENU_ITEMS.iter().position(|&x| x == &food_choice).unwrap();
            // The cost is currently in dollars thus divided by the current value of NEAR to convert it to near
            let cost: f32 = self.menu[&food_choice]; // 5.64
            let client_new = Client {
                table: table_number,
                food: food_choice,
                cost: cost,
            };
            // call the add_client method passing the generated Client and the table_number as key
            self.add_client(client_new.clone(), table_number);
            log!(
                "Your food should cost: {} near",
                self.table_allocation[&table_number].cost
            );
        } else {
            env::log_str("Your item does not exist in our inventory, please try again");
        };
    }

    // non public function to add the newly generated client to the table_allocation hashmap
    #[result_serializer(borsh)]
    fn add_client(&mut self, client: Client, table: u8) {
        self.table_allocation.insert(table, client);
    }
    // Called to get the reciept for the cost of meals per table_number
    pub fn reciept(&self, table_number: u8) {
        log!(
            "You will be charged {} near",
            self.table_allocation[&table_number].cost
        );
    }

    // A payable function where clients can pay for their meals using NEAR tokens
    #[payable]
    pub fn pay(&mut self, table_number: u8) {
        // Assign attached near and the cost of food for the table to variables
        let tokens = env::attached_deposit();
        let charge = self.table_allocation[&table_number].cost;
        log!("deposited {} ", tokens);
        // convert unsigned integer to float and yocto to near
        let token_near = to_near(tokens);
        log!("cost: {}, token near {}", charge, token_near);
        // if checks to compare the token recieved to the expected charge for the meals
        if token_near <= 0.00002 {
            env::log_str("unsuccessful");
            return
        } if token_near + 0.00002 > charge {
            log!(
                "You paid more by {} we hope it's a tip",
                (token_near - charge)
            );
            return;
        } if token_near + 0.00002 < charge {
            log!(
                "You paid less by {} please consider paying up",
                (charge - token_near)
            );
            return;
        } else {
                env::log_str("successful");
                return;
        }
    }
    // Manage client ratings and Restaurant avarage ratings
    pub fn ratings(&mut self, rating: u8, table_number: u8) {
        // Check if the table number making the request is a valid client
        if !self.table_allocation.contains_key(&table_number) {
            log!("sorry only clients can rate our services");
            return;
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
        log!("food Prawns is for {:?}", self.menu.get_key_value("Prawns"));
        log!("food Prawns is for {:?}", self.menu.contains_key("Prawns"))

    }
}

fn to_near(yocto: u128) -> f32 {
         (yocto as f32) / 1_000_000_000_000_000_000_000_000.0
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
    #[test] // test initialization function
    fn restaurant_initialization() {
        let contract = Contract::new();
        assert_eq!(5.0, contract.avg_rating);
    }

    #[test]
    fn create_user() {
        let mut contract = Contract::new();
        contract.order(2, "Prawns".to_string()); // used uppercase to test if its converted to lowercase
        contract.order(4, "Fried egg".to_string());
        assert_eq!(2, contract.table_allocation.len());
        assert_eq!("prawns".to_string(), contract.table_allocation[&2].food); // assert right food is served
        let prawns_cost = 5.0; // price of prawns read directly from the MENU_PRICES array
        assert_eq!(prawns_cost, contract.table_allocation[&2].cost); // check if price is correct for the food ordered
    }
    #[test]
    #[should_panic]
    fn add_ratings_without_table() {
        let mut contract = Contract::new();
        contract.ratings(42, 3); // 2 is client rating and 3 is table number
        assert!(5.0 > contract.avg_rating); // average rating should be lower if the rating is added
    }

    #[test]
    fn add_ratings() {
        let mut contract = Contract::new();
        contract.order(2, "Prawns".to_string());
        assert_eq!(5.0, contract.avg_rating);
        contract.ratings(4, 2); // 4 is client rating and 2 is table number
        assert!(5.0 > contract.avg_rating); // average rating should be lower
    }

    #[test]
    fn pay_test(){
        // fried egg is used which costs 3 dollars or near
        // Paying excess
        let mut contract = Contract::new();
        contract.order(5, "Fried egg".to_string()); 
        let cost = contract.table_allocation[&5].cost;
        let token: u128 = 2 * u128::pow(10, 24); // convert 2 near to equivalent yocto
        let status: i8 = payments::pay_test(token, cost);
        assert_eq!(-1, status);
        // Paying less
        contract.order(5, "Fried egg".to_string());
        let cost = contract.table_allocation[&5].cost;
        let token: u128 = 4 * u128::pow(10, 24); // convert 4 near to equivalent yocto
        let status: i8 = payments::pay_test(token, cost);
        assert_eq!(2, status);
        //  Paying right ammount
        contract.order(5, "Fried egg".to_string());
        let cost = contract.table_allocation[&5].cost;
        let token: u128 = 3 * u128::pow(10, 24); // convert 3 near to equivalent yocto
        let status: i8 = payments::pay_test(token, cost);
        assert_eq!(1, status);
        // Paying 0 near
        contract.order(5, "Fried egg".to_string());
        let cost = contract.table_allocation[&5].cost;
        let token: u128 = 0 * u128::pow(10, 24); // convert 3 near to equivalent yocto
        let status: i8 = payments::pay_test(token, cost);
        assert_eq!(0, status);
    }

    #[test]
    fn test_hash_map() {
        let contract = Contract::new();
        let bools = contract.menu.contains_key(&"prawns".to_string());
        log!("{}", bools);
        assert_eq!(true, bools)
    }
}
