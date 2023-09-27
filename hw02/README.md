# How to operate
## Build

Run `cargo build --release`

## Run
### Diffie-Hellman
To run the server, run `./target/release/hw02 -s`

To run the client, run `./target/release/hw02 -c`

Server can accept more connection in a row (or the same time).
Client can read data from stdin, encrypt them using
stream cipher (PRNG seeded from DH key).
Server processes the message and responds.
Small own (bad) protocol is used for data serialization and transmit.

### Prime product cracking
Run the program without any parameters, just
`cargo run --release`.
This can be really slow in debug variant
(because Rust compiler does no significant optimizations by default 
and adds a lot of additional checks).

## Used algorithms
- Square & Multiply for multiplication mod m
- Sieve of Eratosthenes for prime number finding
- Miller-Rabin prime number test
- Extended Euclidean algorithm

