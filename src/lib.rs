
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, log};

const MENU_ITEMS: [&str;16] = ["Striploin steak", "T-Bone steak", "Boz Centre Cut Fillet", "Tomahawk Ribeye steak", "Fried egg", "Prawns", "Sauteed mushroom", "Onion rings", "Roast baby potatoes", "Sweet potato fries ", "Roast veggies", "Tender greens salad ", "Chocolate Chip Cookie", "Chocolate Cake", "Traditional Apple Pie", "Selection of Ice-cream"];
const MENU_PRICES: [f32; 16] = [40.0, 37.0, 35.0, 75.0, 3.0, 5.0, 4.0, 3.50, 7.0, 6.50, 7.0, 6.50, 8.0, 8.0, 8.50, 2.0, ];

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    table: u8,
    food: String,
    cost: f32,
    rating: i8
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE

    pub fn hello() {
        env::log_str(
            "\n 
            Welcome to Gees reastaurant \n
            Functions are as follows:
            menu: 'menu'
            order: 'order {\"TABLE_NUMBER\", \"food_choice\"}'
            reciept: 'reciept'
            pay: 'pay {\"account-id\"}'
            ratings: 'ratings {\"rating\"}'
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
        ")
    }
    #[init]
    #[result_serializer(borsh)]
    pub fn order(table_number: u8, food_choice: String) -> Self{
        let food: &str = &*food_choice;
        log!("Table number {} your order is {} ", &table_number, &food);
        if MENU_ITEMS.contains(&food) {
            env::log_str("Your order is confirmed");
            let food_index: usize = MENU_ITEMS.iter().position(|&x| x == &food_choice).unwrap();
            let cost: f32 = MENU_PRICES[food_index];
            Self {
                table : table_number,
                food: food_choice,
                cost : cost/50.34,
                rating: 4,
            }
        }else {
            env::log_str("Your does not exist in our inventory, please try again");
            Self {
                table : 0,
                food: String::new(),
                cost : 0.0,
                rating: 4,
            }
        }    
    }

    pub fn reciept(&self) {
        log!("You will be charged {} near", self.cost)
    }

    #[payable]
    pub fn pay(&mut self, account_id: String) {
        let tokens = env::attached_deposit();
        log!("deposited {} from account {}", tokens , account_id);
        let token_float = (tokens as f32)/1000_000_000_000_000_000_000_000.0;
        // let success: bool = payment::main(account_id, self.cost);
        log!("token float: {}, cost: {}", token_float, self.cost);
        if token_float == self.cost {
            env::log_str("successful");
            un_init();
        } if token_float > self.cost {
            log!("You paid more by {} we hope it's a tip", (self.cost)-token_float);

        } if token_float < self.cost {
            log!("You paid less by {} please consider paying up", (self.cost)-token_float);

        }else {env::log_str("unsuccessful please add an amount")}
    }

    pub fn ratings(&mut self, rating: i8) {
        self.rating = rating;
    }
}

fn un_init() {
        Contract {
            table : 0,
            food: String::new(),
            cost : 0.0,
            rating: 4,
        };
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
