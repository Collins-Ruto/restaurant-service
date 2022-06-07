# Commands Restaurants Smart Contract

The smart contract is currently deployed and initialized on:

    restaurants.collinsrutto.testnet

> NOTE: -- represents an integer entry such as `4`, while "__" String entry such as `"Prawns"`
<br>

### Create a new account;

    near deploy --account-id restaurants.collinsrutto.testnet --wasmFile target/wasm32-unknown-unknown/release/rust_template.wasm

### initialize contract :

    near call restaurants.collinsrutto.testnet new --account-id restaurants.collinsrutto.testnet

### Getting instructions for interacting with the smart Contract

    near view restaurants.collinsrutto.testnet hello --account-id collinsrutto.testnet

### Calling for the menu ;

    near view restaurants.collinsrutto.testnet menu --account-id collinsrutto.testnet

### Making an order for a meal ;

    near call restaurants.collinsrutto.testnet order '{"table_number": 3 , "food_choice": "prawns"}' --account-id collinsrutto.testnet

### Ask for your reciept ;

    near view restaurants.collinsrutto.testnet reciept '{"table_number": 3 }' --account-id collinsrutto.testnet

### Make a payment ;

    near call restaurants.collinsrutto.testnet pay --account-id collinsrutto.testnet --deposit 3

### Rate the restaurants's services ;

    near call restaurants.collinsrutto.testnet ratings '{"rating": 5, "table_number": 3}' --account-id collinsrutto.testnet

### View the current number of clients registered on the restaurants

    near view restaurants.collinsrutto.testnet views --account-id collinsrutto.testnet
