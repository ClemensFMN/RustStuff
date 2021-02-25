fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;

    println!("r1 = {}", r1);

    let r3 = &mut s;

    // this works as r1 & r2 are no longer used in the code -> no more immutable borrows -> mutable borrows allowed
    println!("r3 = {}", r3);
    // if we would use r1 after definition of r3, the compiler would throw an error
    //println!("r1 = {}", r1);
}
