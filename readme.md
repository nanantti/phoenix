## How to build for web:

$ cargo build --target wasm32-unknown-unknown

Generates ./target/wasm32-unknown-unknown/debug/phoenix.wasm
Can also be done with the --release tag

## Test the web version locally:

In root, call
$ basic-http-server .

With any browser, visit:

http://127.0.0.1:4000

## Do it in github