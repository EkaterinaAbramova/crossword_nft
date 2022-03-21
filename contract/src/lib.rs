use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen}; // env is used for logging

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
    // Immutable function. 
    #[init] // macro
    pub fn new(solution: String) -> Self { // set the solution once, right after deploying contract. 
        Self {
            crossword_solution: solution,
        }
    }
    
    // Immutable function. 
    pub fn get_solution(&self) -> String {
        self.crossword_solution.clone()
    }

    // Mutable function requires a signed transaction. Now fn returns a bool type (not String)!
    pub fn guess_solution(&mut self, solution: String) -> bool {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        if hashed_input_hex == self.crossword_solution {
            env::log_str("You guessed right!");
            true
        } else {
            env::log_str("Try again.");
            false
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

    #[test] // note the button below 'Run test' (but for some reason runs all tests, not just current one)
    fn debug_get_hash() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build()); // create some basic context for a transaction, then sets up the testing environment.
        // Using a unit test to rapidly debug and iterate
        let debug_solution = "near nomicon ref finance";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string); // Let's debug: "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f"
    }
    
    // This get_context is typically included in all unit tests, i.e. set up a mock context:
    fn get_context(predecessor: AccountId) -> VMContextBuilder { // provide a `predecessor` here, it'll modify the default context
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
    
    #[test]
    fn check_guess_solution() {
        // Get Alice as an account ID
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice);
        testing_env!(context.build());

        // Set up contract object and call the new method
        let mut contract = Contract::new(
            "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f".to_string(), // near nomicon ref finance 69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f
        );
        contract.guess_solution("wrong answer here".to_string());
        assert_eq!(get_logs(), ["Try again."], "Expected a failure log."); //Asserts that two expressions are equal to each other 
        contract.guess_solution("near nomicon ref finance".to_string());
        //assert!(ans, true); // Asserts that a boolean expression is true at runtime.
        assert_eq!( 
            get_logs(), ["Try again.", "You guessed right!"],
            "Expected a successful log after the previous failed log."
        );
    }
}


// ------------------------------------------------- NOTES -------------------------------------------------------
/*
Storage is "paid for" via the native NEAR token. It is not "state rent" but storage staking, paid once, and returned when storage is deleted.

1. Build contract
   Build contrac using command:
     $ ./build.sh

   - In Terminal, run:
      $ near login 
     I logged into my near-ncd.testnet account (note that you are warned about full access being granted)
     near cli generated a private key (kept in jason file on your computer) and public key as a URL param to NEAR wallet
   - To run tests:
      $ cargo test -- --nocapture
2. Create sub-account (or delete and re-create it)
    $ near create-account crossword.near-ncd.testnet --masterAccount near-ncd.testnet
   
   Can view subaccount state:
    $ near state crossword.near-ncd.testnet
   Account crossword.near-ncd.testnet:
      {
        amount: '100000000000000000000000000',
        block_hash: 'CjnJnZRaoyCdh1yW15GicBXDANqYkviw9zacB5svfW4m',
        block_height: 83068600,
        code_hash: '11111111111111111111111111111111',
        locked: '0',
        storage_paid_at: 0,
        storage_usage: 182,
        formattedAmount: '100'
      }
3. Deploy to sub-account
   Ensure the cmd is in the dirctory containing res folder.
    $ near deploy crossword.near-ncd.testnet --wasmFile res/my_crossword.wasm
    See the transaction in the transaction explorer https://explorer.testnet.near.org/transactions/DwkVQ6mQMP2RcGGUG2ygDxUGYG84nXHQyNStF5E4L886 
   
   View state again to see that the contract is now deployed (i.e. code_hash is not 1s):
    $ near state crossword.near-ncd.testnet
   Account crossword.near-ncd.testnet
      {
        amount: '99999816146623589600000000',
        block_hash: 'J5zVXGFgSaquqxgjrKUr9B3ixGjrcPW8n8LVstAyrQAN',
        block_height: 83068685,
        code_hash: '7YqgxU85ADEmRZ43XxKiAEEx9TZ221dsbGFDuHnt8thA',
        locked: '0',
        storage_paid_at: 0,
        storage_usage: 92130,
        formattedAmount: '99.9998161466235896'
      }
4. Interact
   Call new method to set solution as a hashed String (can only call this init method once, second time will be an error)
    $ near call crossword.near-ncd.testnet new '{"solution": "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f"}' --accountId crossword.near-ncd.testnet
   Transaction Id 3BBtntvF1EkNcQWP2AxArZueNpWCCjNALRecqkvHaSbe To see the transaction in the transaction explorer https://explorer.testnet.near.org/transactions/CoBva59CARtGh7tP1vKqQ8ozXrDsU3yDHAJdK75Mfjfm

   Check if argument == solution and store result: 
    $ near call crossword.near-ncd.testnet guess_solution '{"solution": "near nomicon ref finance"}' --accountId near-ncd.testnet
   Receipt: CDANFsib1vyiv9VxkkheCpGUgroyP1GKo9wsJXzPWpXr
   Log [crossword.near-ncd.testnet]: You guessed right!
   Transaction Id 9mbDK8yNLN6eTY94nLVreYEz9jzuysdmm5wHB6YMwLnP To see the transaction in the transaction explorer https://explorer.testnet.near.org/transactions/FU1W1KUoiRNyHkUyeHyiRvSnqTjeCYzkES26eeT5JoK3
5. Delete and re-create sub-account
   This will clear the state and give a fresh start:
    $ near delete crossword.near-ncd.testnet near-ncd.testnet
    $ near create-account crossword.near-ncd.testnet --masterAccount near-ncd.testnet
6. After re-creating account, lets do our deployment and initialisation as a Batch Action (a safer procedure than doing it in 2 steps as we did above):
    $ near deploy crossword.near-ncd.testnet --wasmFile res/my_crossword.wasm --initFunction 'new' --initArgs '{"solution": "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f"}'
   Done deploying and initializing crossword.near-ncd.testnet 

   ----------------------------
7. Getting the simple App to work: 
   Had to in stall 'parcel' as kept getting an error:
    > parcel src/index.html
    sh: parcel: command not found
   In Terminal on Mac ran:
    $ sudo npm install -g parcel-bundler
   After that ran this twice (on the first occasion got an error that could not find React (its a GitHub library of a guy who wrote the puzzles).)
   Ideally it should run the first time:
    $ env CONTRACT_NAME=crossword.near-ncd.testnet npm run start



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