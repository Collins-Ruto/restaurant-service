# Commands Restaurants Smart Contract

The smart contract is currently deployed and initialized on:

    restaurants.collinsrutto.testnet

> NOTE: -- represents an integer entry such as `4`, while "__" String entry such as `"Prawns"`
<br>

### Getting instructions for interacting with the smart Contract

    near view restaurants.collinsrutto.testnet hello --account-id YOUR_ACCOUNT_ID_HERE

### Calling for the menu ;

    near view restaurants.collinsrutto.testnet menu --account-id YOUR_ACCOUNT_ID_HERE

### Making an order for a meal ;

    near call restaurants.collinsrutto.testnet order '{"table_number": __ , "food_choice": "__"}' --account-id YOUR_ACCOUNT_ID_HERE

### Ask for your reciept ;

    near view restaurants.collinsrutto.testnet reciept '{"table_number": __ }' --account-id YOUR_ACCOUNT_ID_HERE

### Make a payment ;

    near call restaurants.collinsrutto.testnet pay --account-id YOUR_ACCOUNT_ID_HERE --deposit __

### Rate the restaurants's services ;

    near call restaurants.collinsrutto.testnet ratings '{"rating": __, "table_number": __}' --account-id YOUR_ACCOUNT_ID_HERE

### View the current number of clients registered on the restaurants

    near view restaurants.collinsrutto.testnet views --account-id YOUR_ACCOUNT_ID_HERE
