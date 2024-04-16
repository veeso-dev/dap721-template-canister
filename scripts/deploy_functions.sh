#!/bin/bash

set -e

deploy_dip721() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  ADMIN_PRINCIPAL="$3"
  SUPPORTED_INTERFACES="$4"
  NAME="$5"
  SYMBOL="$6"
  LOGO="$7"

  echo "deploying dip721 canister"

  if [ -z "$LOGO" ]; then
    LOGO="null"
  else
    LOGO="\"$LOGO\""
  fi

  init_args="(record {
    supported_interfaces = vec { $(for interface in $SUPPORTED_INTERFACES; do echo "variant { $interface }"; done) };
    custodians = vec { principal \"$ADMIN_PRINCIPAL\" };
    logo = $LOGO;
    name = \"$NAME\";
    symbol = \"$SYMBOL\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$init_args" dip721-canister
}

get_arg() {
  read -p "$1: " arg
  if [ -z "$arg" ]; then
    echo "$2"
  else
    echo "$arg"
  fi
}
