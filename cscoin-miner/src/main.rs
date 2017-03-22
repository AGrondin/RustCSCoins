extern crate mersenne_twister;
extern crate crypto;
extern crate rand;
extern crate byteorder;
extern crate itertools;
extern crate openssl;
extern crate serde;
extern crate fnv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate websocket;

//Everything to do with communicating with the server.
mod server_comms;
mod client_miner;

fn main() {
    server_comms::CSCoinClient::create_rsa_keys();
    server_comms::CSCoinClient::load_rsa_keys();
    println!("Hello, world!");
}
