# DIP721

![CI state](https://github.com/veeso-dev/dip721-template-canister/workflows/build-test/badge.svg)

## Introduction

A canister implementing a DIP721 NFT.

## Get started

### Dependencies

Before getting started with ekoke, you need to install these dependencies:

- Rust >= 1.74

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

- Dfx >= 0.16

    ```sh
    sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)
    dfx extension install sns
    ```

- cargo-make

    ```sh
    cargo install cargo-make
    ```

- Wasm32 target

    ```sh
    rustup target add wasm32-unknown-unknown
    ```

### Build canisters

In order to build canister you need to setup the dfx environment and then build the source code, luckily all these steps are automated with cargo-make.

```sh
cargo make dfx-setup
cargo make dfx-build
```

## Changelog

Read [CHANGELOG](./CHANGELOG.md)

## License

You can read the entire license [HERE](LICENSE)
