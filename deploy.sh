#!/bin/sh
./build.sh

#near create-account test2.test-courch.testnet --useAccount test-courch.testnet --initialBalance 4

near deploy courchain-pfe-1.testnet ./target/wasm32-unknown-unknown/release/hello_near.wasm
