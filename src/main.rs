#[macro_use]
extern crate serde_derive;



mod hello_world;
mod tutorial;
mod vec_01;
mod iterator_01;
mod ownership_01;
mod ownership_02;
mod rnd_01;
mod expr;
mod struct_01;
mod closure_1;
mod serde_1;

fn main() {
    println!("Hello, world!");
    hello_world::run_me();
    //tutorial::run_me();
    //vec_01::run_me();
    //iterator_01::run_me();
    //ownership_01::run_me();
    //ownership_02::run_me();
    //rnd_01::run_me();
    //expr::run_me();
    //struct_01::run_me();
    //closure_1::run_me();
    //serde_1::run_me();
}
