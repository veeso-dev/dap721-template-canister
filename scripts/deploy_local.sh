#!/bin/bash

cd "$(dirname "$0")" || exit 1

CANISTER_IDS="../.dfx/local/canister_ids.json"
DIP721_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.dip721-canister.local')"
SUPPORTED_INTERFACES="Mint Burn Approval TransactionHistory"
NAME="DIP721"
SYMBOL="DIP"

source ./deploy_functions.sh
source ./did.sh

ADMIN_PRINCIPAL="$(dfx identity get-principal)"

dfx stop
dfx start --background

cd ../

set -e

deploy_dip721 "reinstall" "local" "$DEFERRED_PRINCIPAL" "$SUPPORTED_INTERFACES" "$NAME" "$SYMBOL"

set +e

dfx stop

exit 0
