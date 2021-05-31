# ethers.js vs ethers-rs mempool stream performance

Based on these tests Rust appears to be approximately 62.67% faster for mempool streaming than Typescript on the same hardware and node connection

## Installation

```
cargo build
```

## Run

Start server
```
cargo run
```

## Benchmarking against Typescript code

Install Typescript
```
cd benchmarks
yarn install
```

Compare benchmarks for processing over 60 seconds
```
# Run Typescript for 60 seconds
ts-node pending-stream.ts & sleep 60 ; kill $!

# Run equivalent Rust code for 60 seconds
./target/release/pending-stream & sleep 60 ; kill $!
```