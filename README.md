# UBIKChain node

This is blockchain node software for [UBIK.hr](https://ubik.hr), a Croatian self-regulating NGO dealing with blockchain and cryptocurrency education and adoption.

The currency is defined as UBIK, using 12 decimals with an SS58 format of 2 (matching Kusama addresses).

## Running

You need Rust installed. Instructions here: https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/setup or just run `curl https://getsubstrate.io -sSf | bash -s -- --fast`

```bash
git clone https://github.com/swader/ubikchain --depth 1
cd ubikchain
./scripts/init.sh
cargo build --release
./target/release/ubikchain-node
```

## Included features

- Utility (batch tx)
- Vesting (vested transfer, gradual unlock of funds)

## Joining as a validator

TBD

## Contributing

PRs are welcome.
