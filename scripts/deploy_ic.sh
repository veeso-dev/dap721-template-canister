#!/bin/bash

cd "$(dirname "$0")" || exit 1

source ./deploy_functions.sh
source ./did.sh

ADMIN_PRINCIPAL="$(dfx identity get-principal)"
SUPPORTED_INTERFACES="Mint Burn Approval TransactionHistory"

CANISTER="$1"

if [ -z "$CANISTER" ]; then
  echo "Please provide the canister name as an argument"
  echo "Available canisters:"
  echo "- dip721"
  exit 1
fi

set -e
dfx identity use ekoketoken

cd ../

case "$CANISTER" in

  "dip721")
    NAME=$(get_arg "name" "DIP721")
    SYMBOL=$(get_arg "symbol" "DIP")
    LOGO=$(get_arg "logo")
    deploy_deferred "reinstall" "ic" "$ADMIN_PRINCIPAL" "$SUPPORTED_INTERFACES" "$NAME" "$SYMBOL" "$LOGO"
    ;;
  
  *)
    echo "Invalid canister name"
    echo "Available canisters:"
    echo "- dip721"
    exit 1
    ;;

esac

set +e

exit 0
