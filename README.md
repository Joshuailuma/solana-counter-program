# Counter program in Anchor
This project demonstrates the basics of Anchor programs. A counter account is created and the value in the counter field is increased or decreased (depending on the function called) only if the authority has signed the transaction.

## Functions

- `pub fn create(ctx: Context<Create>)` creates a counter account and initializes its count to 0

- `pub fn increment(ctx: Context<Increment>)` increments the count by 1, only if the authority has signed the transaction

- `pub fn decrement(ctx: Context<Increment>)` decrements the count by 1, only if the authority has signed the transaction

## Running the program
-  ` npm i` to install the dependencies
- `anchor run test` to run the test script on the localnet

## Author
Joshua Iluma