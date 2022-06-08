# Commands Restaurants Smart Contract

The smart contract is currently deployed and initialized on:

    restaurants.collinsrutto.testnet

> NOTE: -- represents an integer entry such as `4`, while "\_\_" String entry such as `"Prawns"` > <br>

### Create a new account;

    near deploy --account-id restaurants.collinsrutto.testnet --wasmFile target/wasm32-unknown-unknown/release/rust_template.wasm

### Getting instructions for interacting with the smart Contract

    near view restaurants.collinsrutto.testnet hello --account-id collinsrutto.testnet

### Calling for the menu ;

    near call restaurants.collinsrutto.testnet menu --account-id collinsrutto.testnet

### Making an order for a meal ;

    near call restaurants.collinsrutto.testnet order '{"table_number": 3 , "food_choice": "prawns"}' --account-id collinsrutto.testnet

### Ask for your reciept ;

    near view restaurants.collinsrutto.testnet reciept '{"table_number": 3 }' --account-id collinsrutto.testnet

### Make a more payment ;

    near call restaurants.collinsrutto.testnet pay '{"table_number": 3}' --account-id collinsrutto.testnet --deposit 7

### Make a exact payment ;

    near call restaurants.collinsrutto.testnet pay '{"table_number": 3}' --account-id collinsrutto.testnet --deposit 5

### Make a less payment ;

    near call restaurants.collinsrutto.testnet pay '{"table_number": 3}' --account-id collinsrutto.testnet --deposit 3

### Rate the restaurants's services ;

    near call restaurants.collinsrutto.testnet ratings '{"rating": 3, "table_number": 3}' --account-id collinsrutto.testnet
### Rate the restaurants's services fake table ;

    near call restaurants.collinsrutto.testnet ratings '{"rating": 5, "table_number": 1}' --account-id collinsrutto.testnet

