# Restaurant service smart contract rust

## About 

This is a restaurant service contract where clients can get to a restaurant and get the restaurant's
near account. With it, they can check the menu, make food orders, add ratings and also pay for their food items.
This can all be done securely and anoymously by leveraging on the characteristics of blockchains.
<br/>
A demo of how everything works can be found here [Loom video](https://www.loom.com/share/f139740afc5f4cab81b97b77922dd5b8)
<br>

## Initialization for dependencies needed <br/>
Borsh is the recommended serialization method for near smart contract development.

    use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
    use near_sdk::{env, log, near_bindgen};
    use std::collections::HashMap;


External modules for handling the information to be shown to the user on call of the 'hello' function. The payments module contains the testing function for the payments method.

    mod info;
    mod payments;


HashMaps are used to store multiple sessions of different clients on different unique tables.
A list of food choices that the restaurant offers and prices for each also here
Table numbers are therefore used as keys.
All_ratings stores all ratings made by clients and the avg_rating stored the final computed average rating.

    pub struct Contract {
        menu: HashMap<String, f32>,
        table_allocation: HashMap<u8, Client>,
        all_ratings: Vec<u8>,
        avg_rating: f32,
    }

This is all the clients information we'll be storing per table in the contract.table_allocation, that is the food the client requested, the table and also the cost of the food in NEAR.

    pub struct Client {
        ...//initialization here
    }

Initialization function for the contract and setting the initial avg_rating.

    impl Default for Contract {
        fn default() -> Self {
            Self {
                menu: info::menu_items(),
                table_allocation: HashMap::new(),
                all_ratings: Vec::new(),
                avg_rating: 5.0,
            }
        }
    }
## Implimentation of the Contract struct

    impl Contract {
        ...// implimentation here
    }

Call to get initial instructions on how to use contract with the current restaurant avarage ratings.

    pub fn hello(&self) {
        info::hello(self.avg_rating);
    }
 
## order food order function

The order function with which the client calls to make an meal order. The client passes his food choice and table number.
The data provided is used to initialized a new instance of a client object only if the food provided exists in the Contract.menu HashMap.

    pub fn order(&mut self, table_number: u8, food_choice: String) {
        ...// initialization
    }

<details>
<summary>
    Detailed description
</summary>
<br/>

    pub fn order(&mut self, table_number: u8, food_choice: String) {
        // convert food_choice from string to str so that it can be used to iterate through MENU_ITEMS
        // let food: &str = &*food_choice;
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

</details> <br>

## Payment Method

A payable function where clients can pay for their meals using NEAR tokens with near attached to the function call.

    #[payable]
    pub fn pay(&mut self, table_number: u8) {
        ...// initialization
    }

<details>
<summary>
Detailed description
</summary>
<br/>

    #[payable]
    pub fn pay(&mut self, table_number: u8) {
        // Assign attached near and the cost of food for the table to variables
        let tokens = env::attached_deposit();
        let charge = self.table_allocation[&table_number].cost;
        log!("deposited {} ", tokens);
        // convert unsigned integer to float and yocto to near
        let token_near = to_near(tokens);
        log!("cost: {}, token near {}", charge, token_near);
        // if checks to compare the token recieved to the expected charge for the meals and give relevant feedback
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

</details><br>

## Manage client ratings and Restaurant avarage ratings

    pub fn ratings(&mut self, rating: u8, table_number: u8) {
        ...// initialization
    }
<details>
<summary>
get more details
</summary>
<br>

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

</details> <br>

> Convert the Yocto used for NEAR transactions to near currency 

    fn to_near(yocto: u128) -> f32 {
            (yocto as f32) / 1_000_000_000_000_000_000_000_000.0
    }
## Unit tests

    #[cfg(test)]
    mod tests {
        use super::*;

Test the new and order methods agains Prawns and Fried egg whose properties are predefined struct contract.menu.

    #[test]
    fn create_user_1() {
        let mut contract = Contract::new();
        contract.order(2, "Prawns".to_string());
        contract.order(4, "Fried egg".to_string());
        assert_eq!(2, contract.table_allocation.len());
        // assert right food is served
        assert_eq!("Prawns".to_string(), contract.table_allocation[&2].food); 
        let prawns_cost = 5.0; // price of prawns read directly from the MENU_PRICES array
        // assert if price is correct for the food ordered
        assert_eq!(prawns_cost, contract.table_allocation[&2].cost);
    }
<br>
Here we test that no one should be able to rate our restaurant if not a valid client with a registered table allocation.
The test is expected to fail as the ratings should not change.

    #[test]
    #[should_panic]
        fn add_ratings_without_table() {
            let mut contract = Contract::new();
            contract.ratings(42, 3); // 2 is client rating and 3 is table number
            assert!(5.0 > contract.avg_rating); // average rating should be lower if the rating is added
        }
We test if when a valid client rating the restaurant there's change on the restaurants average rating.

    #[test]
    fn add_ratings() {
        let mut contract = Contract::new();
        contract.order(2, "Prawns".to_string());
        assert_eq!(5.0, contract.avg_rating);
        contract.ratings(4, 2); // 4 is client rating and 2 is table number
        assert!(5.0 > contract.avg_rating); // average rating should be lower
    }

Due to it being payable, the tests fo pay are generic and depending on the external module `mod payments::pay_test()` function for simulation purposes.\
We call the `mod payments::pay_test()` with various "NEAR" to simulate less, more, excact and method call without attached near.\
The function responds with "status codes" for each: `-1` for less, `0` for none, `1` for equal and `2` for excess.

    #[test]
        fn pay_test(){
        // fried egg is used which costs 3 dollars or near
        // Paying excess
            let mut contract = Contract::new();
            contract.order(5, "Fried egg".to_string()); 
            let token: u128 = 2 * u128::pow(10, 24); 
            // convert 2 near to equivalent yocto
            let cost = contract.table_allocation[&5].cost;
            let status: i8 = payments::pay_test(token, cost);
            assert_eq!(-1, status);
<details>
<summary>
More of the Payment tests...
</summary>
<br/>

    // Paying less
        contract.order(5, "Fried egg".to_string());
        let token: u128 = 4 * u128::pow(10, 24); 
        // convert 4 near to equivalent yocto
        let cost = contract.table_allocation[&5].cost;
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

</details> <br>

A pre-written list of commands to excecute on this contract can be found here:

> [Restaurant service tests](./src/commands/)

or on my gist: 

> [Restaurant service tests gist]()

Built with the [Near Rust Template ](https://github.com/near/near-sdk-rs#pre-requisites)

<!-- 8. Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release` -->

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)
