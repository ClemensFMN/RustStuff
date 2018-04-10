

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json;
use serde_json::Value;

pub fn run_me() {

	let mut core = Core::new().unwrap();
	let client = Client::new(&core.handle());

	let uri = "http://httpbin.org/ip".parse().unwrap();
	/*
	let work = client.get(uri).and_then(|res| {
	    println!("Response: {}", res.status());

	    res.body().for_each(|chunk| {
	        io::stdout()
	            .write_all(&chunk)
	            .map_err(From::from)
	    })
	});
	*/

	let work = client.get(uri).and_then(|res| {
	    println!("Response: {}", res.status());

	    res.body().concat2().and_then(move |body| {
	        let v: Value = serde_json::from_slice(&body).map_err(|e| {
	            io::Error::new(
	                io::ErrorKind::Other,
	                e
	            )
	        })?;
	        println!("current IP address is {}", v["origin"]);
	        Ok(())
	    })
	});

	core.run(work).unwrap();
}
