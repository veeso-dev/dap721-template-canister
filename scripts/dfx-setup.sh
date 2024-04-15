#!/bin/bash

dfx stop
dfx start --background --clean
dfx canister create dip721-canister

dfx stop
