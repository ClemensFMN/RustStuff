#[macro_use]
extern crate log; // the log crate is just a facade,
use simple_logger::SimpleLogger; // we need a "real" logging implementation, otherwise logging does NOT work
// SimpleLogger is not the only solution (see log crate docs but a simple one :-))

fn main() {
    SimpleLogger::new().init().unwrap(); // init SimpleLogger

    println!("Hello, world!");
    debug!("Hello"); // actually log something
    warn!("Hello");
}
