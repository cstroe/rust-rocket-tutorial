# rust-rocket-tutorial

This is me messing around with [Rust](https://www.rust-lang.org) and the [Rocket](https://rocket.rs/guide/) web framework for Rust.

## Setup

You need Rust nightly for this project:

```
rustup toolchain install nightly
rustup default nightly
```

## Running

Just use Cargo:

```
$ cargo run
   Compiling rust-rocket-tutorial v0.1.0 (file:///home/cosmin/Zoo/02-rust/helpful)
    Finished dev [unoptimized + debuginfo] target(s) in 2.29s
     Running `target/debug/rust-rocket-tutorial`
🔧  Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 8
    => secret key: generated
    => limits: forms = 32KiB
    => tls: disabled
🛰  Mounting '/':
    => GET /status
🚀  Rocket has launched from http://localhost:8000
```

Then, in a separate terminal:

```
$ curl http://localhost:8000/status
{"status":"Ok","hello":"world"}
```

## Testing

To run the tests:

```
$ cargo test
   Compiling rust-rocket-tutorial v0.1.0 (file:///home/cosmin/Zoo/02-rust/helpful)
    Finished dev [unoptimized + debuginfo] target(s) in 2.60s
     Running target/debug/deps/rust_rocket_tutorial-4f437e9e7ae024b7

running 1 test
test test::hello_world ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```


