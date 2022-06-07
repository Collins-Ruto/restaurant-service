use near_sdk::{env, log};

#[allow(dead_code)]
pub fn pay_test(tokens: u128, cost: f32) -> i8 {
    // Assign attached near and the cost of food for the table to variables
    let charge = cost;
    log!("deposited {} ", tokens);
    // convert unsigned integer to float and yocto to near
    let token_near = to_near(tokens);
    log!("cost: {}, token near {}", charge, token_near);
    // if checks to compare the token recieved to the expected charge for the meals
    if token_near <= 0.00002 {
        env::log_str("unsuccessful");
        return 0
    } if token_near - 0.00002 > charge {
        log!("You paid more by {} we hope it's a tip",(token_near - charge)
        );
        return 2
    } if token_near + 0.00002 < charge {
        log!("paid less by {}", (charge - token_near)
        );
        return -1
    } else {
        env::log_str("successful");
        return 1
    }
}

fn to_near(yocto: u128) -> f32 {
         (yocto as f32) / 1_000_000_000_000_000_000_000_000.0
}