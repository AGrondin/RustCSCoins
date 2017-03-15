
extern crate openssl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate websocket;

//Everything to do with communicating with the server.
mod server_comms;

fn main() {
    println!("Hello, world!");
}
