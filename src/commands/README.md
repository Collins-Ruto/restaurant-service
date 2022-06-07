# Commands Restaurant Smart Contract

The smart contract is currently deployed and initialized on:

    restaurant.collinsrutto.testnet

> NOTE: -- represents an integer entry such as `4`, while "__" String entry such as `"Prawns"`
<br>

### Getting instructions for interacting with the smart Contract

    near view restaurant.collinsrutto.testnet hello --account-id YOUR_ACCOUNT_ID_HERE

### Calling for the menu ;

    near view restaurant.collinsrutto.testnet menu --account-id YOUR_ACCOUNT_ID_HERE

### Making an order for a meal ;

    near call restaurant.collinsrutto.testnet order '{"table_number": __ , "food_choice": "__"}' --account-id YOUR_ACCOUNT_ID_HERE

### Ask for your reciept ;

    near view restaurant.collinsrutto.testnet reciept '{"table_number": __ }' --account-id YOUR_ACCOUNT_ID_HERE

### Make a payment ;

    near call restaurant.collinsrutto.testnet pay --account-id YOUR_ACCOUNT_ID_HERE --deposit __

### Rate the restaurant's services ;

    near call restaurant.collinsrutto.testnet ratings '{"rating": __, "table_number": __}' --account-id YOUR_ACCOUNT_ID_HERE

### View the current number of clients registered on the restaurant

    near view restaurant.collinsrutto.testnet views --account-id YOUR_ACCOUNT_ID_HERE
