#[macro_use]

extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate futures;
extern crate hyper;
extern crate tokio_core;


mod rnd_01;
mod serde_1;
mod serde_2;
mod hyper_client;


fn main() {
    println!("Hello, world!");

    //rnd_01::run_me();
    
    //serde_1::run_me();
    //serde_2::run_me();
    hyper_client::run_me();
}
