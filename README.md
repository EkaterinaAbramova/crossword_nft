# NEAR Blockchain : Rust Smart Contract Crossword NFT

This contract is written in Rust programming language, and was developed using the Visual Studio Code IDE.

## Overview

1. NFT was built following [NEAR: Zero-to-Hero](https://www.near-sdk.io/zero-to-hero/basics/overview) instructions.
2. Set up the [prerequisites](https://github.com/near/near-sdk-rs#pre-requisites).

## Explore Contract
The source for this contract is in `contract/src/lib.rs`. The contract contains 1 puzzle which should be solved by the user.

### Build Contract
Since the smart contract is a library (crate) rather than a binary, the `cargo run` command is unavailable. Instead to build the contract ensure your cmd is in the `knn_nft` path and use:
```bash
./build.sh
```
If you run into 'permission denied' difficulties, simply execute this line first:
```bash
chmod +x build.sh
```

### Test Contract
There are unit tests in this contract designed to verify that the contract code is working as indtended. Following completion of test execution, check that all tests passed (last returned statement on the command line).
```bash
cargo test -- --nocapture
```

### Create Account
This smart contract is intended to be interacted with using the development account. Go to [NEAR URL](https://wallet.testnet.near.org) and create a testnet account, for example called `myacc.testnet`. For help see [Dev Accounts](https://docs.near.org/docs/concepts/account#dev-accounts). Following this run the command below, which lets the near cli generate a private key, kept in the jason file on your computer, and public key as a URL parameter to NEAR wallet by logging into your account from your Terminal (browswer opens up):
```bash
near login
```

Next, using the Terminal create a sub-account (for example `contract.myacc.testnet`) to which the contract will be deployed to (this is the best practice for deploying contracts), using command:
```bash
near create-account crossword.myacc.testnet --masterAccount myacc.testnet
```

### View Subaccount State
You can view the current state of the subaccount (to which we will deploy the contract to later on):
```bash
near state crossword.myacc.testnet
```
You will see something like this, with your own data, however note that the `code_hash` contains all 1s, therefore we know that no contract is currenntly deployed to this subaccount.
```
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
```

### Deploy The Contract to Blockchain
Ensure the cmd is in the dirctory containing `res` folder and run the following command, which will deploy theh contract to blockchain and at the same time initialise contract parameter `solution` to the hashed solution (as a string). Note that while test account that you created does not have real tokens, the contract deployment is done to an actual blockchain.
```bash
near deploy crossword.myacc.testnet --wasmFile res/my_crossword.wasm --initFunction 'new' --initArgs '{"solution": "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f"}'
```

### View Subaccount State Again
You can view the current state of the subaccount (on which the contract is now deployed):
```bash
near state crossword.myacc.testnet
```
This time you will see that the `code_hash` is not all 1s, therefore a contract has been deployed successfully:
```
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
```

### Interact With The Contract
#### Check if argument == solution and store result: 
```bash
near call crossword.myacc.testnet guess_solution '{"solution": "near nomicon ref finance"}' --accountId myacc.testnet
```

### Create an Interactive Frontend
```bash
env CONTRACT_NAME=crossword.myacc.testnet npm run start
```
If you get an error 
```bash
> parcel src/index.html
sh: parcel: command not found
```
In Terminal on Mac run:
```bash
$ sudo npm install -g parcel-bundler
```
After that you may need to run the original command twice (on the first occasion you may get an error about 'could not find React' (it is a GitHub library of a developer who wrote the puzzles)). So run below twice if necessary:
```bash
env CONTRACT_NAME=crossword.myacc.testnet npm run start
```
After this command executes successfuly, command line will output website name: http://localhost:1234 Go to this website and the dApp opens up.
You are ready to guess the crossword!

[Correct solution is: near nomicon ref finance]

For more informationn see:
* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)
