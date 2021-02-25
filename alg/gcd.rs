fn euclidean_gcd(mut a:i32, mut b:i32) -> i32 {
    while(b != 0) {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    println!("Hello");
    let res = euclidean_gcd(20, 14);
    println!("{}", res);
}
