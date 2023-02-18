# Rust Smart Contracts for WebAssembly (WASM)

This repository contains implementations for WebAssembly (WASM) smart contracts written in Rust for different blockchain platforms. The primary goal of this project is to compare the ergonomics and other aspects of developing smart contracts in Rust on different blockchain platforms.

## Supported Blockchains

The following blockchain platforms are currently supported:

- NEAR Protocol

## RevocationList2020 Specification

All smart contracts in this repository implement the RevocationList2020 specification. The RevocationList2020 specification provides a way to revoke the credentials issued by a specific issuer. This can be useful in scenarios where an issuer needs to revoke a credential due to various reasons, such as a user's account being compromised or the credential being no longer valid.

The RevocationList2020 specification provides a standard format for revocation lists and defines how these lists can be used to revoke credentials issued by an issuer. All smart contracts in this repository adhere to this specification, which ensures that the revocation process is interoperable across different blockchain platforms.

By implementing the RevocationList2020 specification, these smart contracts provide an additional layer of security and flexibility, making them suitable for various use cases.

## Getting Started

To get started with this project, you'll need to have Rust and the toolchains for the respective blockchain platforms installed on your machine.

1. Clone the repository:

```sh
git clone https://github.com/noandrea/revocation-lists-2020-contracts.git
```

2. Change into the project directory:
```sh
Copy code
cd rust-smart-contracts
```

3. Change into the directory for the blockchain platform you want to work with:

```sh
cd ethereum
```

4. Build the smart contract:

```sh
cargo build --target wasm32-unknown-unknown --release
```

4. Deploy the smart contract to the blockchain platform:

For Ethereum, you can use tools such as `web3.js` or `ethers.js` to deploy the contract. For Solana, you can use `solana deploy`. For NEAR Protocol, you can use `near deploy`.

## Contributing
We welcome contributions from anyone. If you'd like to contribute to this project, please fork the repository and create a pull request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.