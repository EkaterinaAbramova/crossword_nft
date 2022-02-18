use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen}; // env is used lower for logging

// constant for puzzle number
const PUZZLE_NUMBER: u8 = 1; // IN-MEMORY STORAGE (not paid for; value 1 is contained in the contract code)

// ------------------------------------------ CONTRACT STATE --------------------------------------------------
#[near_bindgen] // macro used on a struct and fn implementations to generate code to be a valid NEAR contract and expose intended fns for external callability.
#[derive(Default, BorshDeserialize, BorshSerialize)] // Borsh: Binary Object Representation Serializer for Hashing to convert code to 0,1 efficiently.
pub struct Contract {
    // struct is public so other code can use it, but the fields inside are private (no mut)
    crossword_solution: String, // PERSISTENT STORAGE (STAKING REQUIRED)
}

// ------------------------------------------ CONTRACT METHODS --------------------------------------------------
#[near_bindgen]
impl Contract { // impl provides methods on structs and enums
    // Immutable functions.
    pub fn get_puzzle_number(&self) -> u8 { // user can call this function without signing a transaction, since its view-only (like a GET request but using RPC endpoints)
        PUZZLE_NUMBER // don't need a 'return' 
    }
    // Mutable function requires a signed transaction. When smart contract is called, the contract's field crossword_solution will change.
    pub fn set_solution(&mut self, solution: String) { 
        self.crossword_solution = solution; // user (devloper actually!) supplies a parameter 
    }
    // Mutable function requires a signed transaction.
    pub fn guess_solution(&mut self, solution: String) { 
        if solution == self.crossword_solution {
            env::log_str("You guessed right!") } 
        else {
            env::log_str("Try again.")
        }
    } 
}

// ---------------------------------------------- TESTS ----------------------------------------------------------
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

// ------------------------------------------------- NOTES -------------------------------------------------------
/*
NEAR:
- Blockchain is an open ledger, so everyone can see the state of smart contracts and transactions taking place.
- Storage is "paid for" via the native NEAR token. It is not "state rent" but storage staking, paid once, and returned when storage is deleted.


CONTRACT:
1. Build contract
    $ ./build.sh

   - In Terminal, run:
      $ near login 
     near cli generated private key (kept in jason file on computer) and public key as a URL param to NEAR wallet -> browser opens up, log into the testnet account.
   - To run tests:
      $ cargo test --package rust-template -- --nocapture
2. Create sub-account (or delete and re-create it)
    $ near create-account crossword.drkat.testnet --masterAccount drkat.testnet
   
   Can view subaccount state:
    $ near state crossword.drkat.testnet
   Account crossword.drkat.testnet:
      {
        amount: '100000000000000000000000000', 
        block_hash: '9yskE7TXpYFzXwpk15ABEwMMp6Kczcxe1eKDCsUEmFFv',
        block_height: 82937991,
        code_hash: '11111111111111111111111111111111', \\ codehash is all 1s means no contract is not deployed to this account.
        locked: '0',
        storage_paid_at: 0,
        storage_usage: 182,
        formattedAmount: '100'
      }
3. Deploy to sub-account
   Ensure the cmd is in the dirctory containing res folder.
    $ near deploy crossword.drkat.testnet --wasmFile res/my_crossword.wasm
    See the transaction in the transaction explorer https://explorer.testnet.near.org/transactions/DwkVQ6mQMP2RcGGUG2ygDxUGYG84nXHQyNStF5E4L886 
   
   View state again to see that the contract is now deployed (i.e. code_hash is not 1s):
    $ near state crossword.drkat.testnet
   Account crossword.drkat.testnet
      {
        amount: '99999816195677182400000000',
        block_hash: '5numAYbTzgETD3bQ1oyh4vPVofJ3D6YLixwrSPiwwG9Q',
        block_height: 82939597,
        code_hash: '8vfaZYFXWsFe4UJoXZWGLcgyybXziKEsGDVYx7QjkYad', \\ codehash is NOT 1s means contract IS deployed to this account.
        locked: '0',
        storage_paid_at: 0,
        storage_usage: 92094,
        formattedAmount: '99.9998161956771824'
      }
4. Interact
   Call view-only method (no params so don't pass any but can supply empty input too; i.e. equivalent ways of calling the function):
    $ near view crossword.drkat.testnet get_puzzle_number
    $ near view crossword.drkat.testnet get_puzzle_number '{}'
   
   Call set_solution method to set the solution as a String
    $ near call crossword.drkat.testnet set_solution '{"solution": "near nomicon ref finance"}' --accountId drkat.testnet
   Transaction Id CoBva59CARtGh7tP1vKqQ8ozXrDsU3yDHAJdK75Mfjfm To see the transaction in the transaction explorer https://explorer.testnet.near.org/transactions/CoBva59CARtGh7tP1vKqQ8ozXrDsU3yDHAJdK75Mfjfm

   Check if argument == solution and store result: 
    $ near call crossword.drkat.testnet guess_solution '{"solution": "near nomicon ref finance"}' --accountId drkat.testnet
   Receipt: 4GCtgWLWtAR9VA1A9Ad6m3vaXQVaH8Lvw5SKxwyyAbaX
   Log [crossword.drkat.testnet]: You guessed right!
   Transaction Id FU1W1KUoiRNyHkUyeHyiRvSnqTjeCYzkES26eeT5JoK3 To see the transaction in the transaction explorer https://explorer.testnet.near.org/transactions/FU1W1KUoiRNyHkUyeHyiRvSnqTjeCYzkES26eeT5JoK3
5. Delete and re-create sub-account
   This will clear the state and give a fresh start:
    $ near delete crossword.friend.testnet friend.testnet
    $ near create-account crossword.friend.testnet --masterAccount friend.testnet
  
RUST:
- Indent code shortcut: cmd + ] 
- In Rust by default everything (all variables) is PRIVATE!!! Need to use &mut to ensure can change values of variables.
- Rust is a statically typed.
- Indexing starts from 0.
- i32 is default integer.
- f64 is default float.
- '' char literals.
- "" string literals.
- Syntax 1_000 means integer 1000.
- Compiling in release mode won't check for integer overflow!
- Rust won't auto convert non-Boolean types to a Boolean for if statements. 
- Structs and enums are the building blocks for creating new types.
- Structs - custom data type that lets you name and package together multiple related values.
- Structs and enums have data
- #[expr] is an outer attribute (specifying attributes on an item)

Fundamental data types:
    scalar types: integers, floating-point numbers, Booleans (true/false), characters.
    primitive compound types: 
        tuples 
        arrays:  all elems same type; fixed length (# elems doesn't change); [1,2,3]. Allocated on stack.

Std Lib:
    vector: allowed to grow.

Expressions do not include ending semicolons.
{
    let x = 3;
    x + 1 // if put ; at the end here, will change expression to a statement. 
}
Statements don’t evaluate to a value.

Fns return the last EXPRESSION implicitly (no need for 'return').
We don’t name return values.
MUST declare return value's type after an arrow (->)
fn five() -> i32 {
    5
}
Funciton names follow snake convention by style guide my_funciton_name.
It is not typical to have getter methods (on structs) in Rust.
*/