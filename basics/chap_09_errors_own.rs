
fn f1(i:i32) -> i32{
    i+1
}

// define a function which simulates some I/O whatever operation. Dependong on the provided value, the function "succeeds" or fails
fn f2(i:i32) -> Result<i32, i32> {
    let out = if i<10 { Ok(i) } else {Err(i)};
    out
}

// to demonstrate the ? operator, we define some intermediate function. that way we do not need to change the type of the main function
fn use_f2(i:i32) -> Result<i32, i32> {
   // From the Rust Book (https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
   //If the value of the Result is an Ok, the value inside the Ok will get returned from this expression,
   // and the program will continue. If the value is an Err, the Err will be returned from the whole 
   // function as if we had used the return keyword so the error value gets propagated to the calling code.
   let res = f2(i)?;
   Ok(res)
}



fn main() -> () {
    println!("{}", f1(3));
    println!("{:?}", f2(3));
    println!("{:?}", f2(15));

    // option 1
    // now cal the function and process the result with a match expression
    let res1 = f2(3);
    let res1 = match res1 {
        Ok(val) => val,	// success -> unpack value from the Ok
        Err(val) => panic!("did not work, value: {}", val), // failure -> panic
    };
    println!("Option 1, successful call, got {:?}", res1);

    // Option 2a
    let res2a = f2(3);
    let res2a = res2a.expect("did not work");
    println!("Option 2a, successful call, got {:?}", res2a);

    // Option 2
    let res2 = f2(3);
    let res2 = res2.unwrap();
    println!("Option 2, successful call, got {:?}", res2);

    // Option 3
    let res3 = f2(3);
    let res3 = res3.unwrap_or_else(|err| {
        panic!("did not work, error {:?}", err);
    });
    println!("Option 3, successful call, got {:?}", res3);

    // Option 4, using ? as some kind of shortcut
    let res4 = use_f2(3).unwrap();
    println!("Option 4, successful call, got {:?}", res4);
    

}
