
// trying to express an expression tree
enum Expr {
    Val(i32),
    // this does not work as it creates a possibly infinite struct and this cannot be allocated at
    // compile time
    //Sum(Expr, Expr)
    // and representing graphs seems to be rather complicated :-(
}

fn main() {
    println!("Hello from expr");
}
