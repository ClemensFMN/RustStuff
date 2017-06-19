// ---------------- Fibonacci ----------------
struct Fib {
    curr: u32,
    next: u32
}

// and an Iterator implementation which calculates the next value
impl Iterator for Fib {
    type Item = u32;    // the type we are iterating over; this must match the return type of next!!
    fn next(&mut self) -> Option<u32> { // the method calculating the next value
        let temp = self.curr + self.next;
        self.curr = self.next;
        self.next = temp;
        Some(self.curr) // we never stop -> always return Some; note the missing semicolon -> this is a return value
    }
}

// ---------------- Version 2 with max value ----------------

struct FibMax {curr: u32, next: u32, maxval: u32} // hold the state + the maximum value

impl Iterator for FibMax {
    type Item = u32;    // the type we are iterating over; this must match the return type of next!!
    fn next(&mut self) -> Option<u32> {
        let temp = self.curr + self.next;
        self.curr = self.next;
        self.next = temp;
        if self.curr < self.maxval { // if we are below the maxvalue
            Some(self.curr)} // return the next Fibonacci number
        else {
            None // otherwise return None to indicate that the Iterator is finished
        }
    }
}

// ---------------- Collatz ----------------
struct Collatz {xn: u32}

impl Iterator for Collatz {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        //println!("xn = {}", self.xn);
        let temp = self.xn;
        if temp % 2 == 0 { //even
            self.xn = temp/2;
            Some(temp)}
        else if temp % 2 == 1 { //odd
            self.xn = 3*temp+1;
            Some(temp)}
        else {
            None
        }
    }
}

// ---------------- Primes ----------------
struct Prime {init: u32}

fn divisible_by(a: u32, b: u32) -> bool {
    a % b == 0
}

fn isprime(num: u32) -> bool {
    // simple primality test; I think we can optimise the range to sqrt(num)?
    for i in 2..num {
        if divisible_by(num, i) {
            return false
        }
    }
    true
}

impl Iterator for Prime {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        loop {
            self.init += 1;
            if isprime(self.init) {
                return Some(self.init);
            }
        }
    }
}


// ---------------- Prime Factorization ----------------
// first take, probably slow as hell because we generate the prime numbers to divide through on the fly
fn primefact_v1(num: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut temp = num;
    
    let p = Prime{init:1};
    // start with prime 2, 3, 5
    for pn in p {
        //println!("pn = {}", pn);
        // and subsequently divide by the _same_ prime number until this is no longer possible (no zero remainder)
        while divisible_by(temp, pn) {
            temp = temp / pn;
            //println!("temp = {}", temp);
            // collect the primes
            factors.push(pn);
        }
        // either we are done and return the factors
        if temp == 1 {
            return factors;
        }
        // and continue with the next prime
    }
    factors  // this feels a bit akward as we will never the function via this line, but it is required because of the type signature
}



pub fn run_me() {
    let f = Fib{curr:1, next:1};
    for i in f.take(10) {
        println!("{}", i);
    }

    // sum over the even Fibonacci numbers smaller than 4 million (Project Euler #2)
    let f2 = Fib{curr:1, next: 1};
    let res = f2.take_while(|&i| i < 4000000).filter(|&i| i%2 == 0).fold(0, |acc,i| acc + i);
    println!("Sum: {}", res);
    
    let fmax = FibMax{curr:1, next:1, maxval: 200};
    for i in fmax {
        println!("{}", i);
    }

    println!("XXXXXXXXXXX");
    
    let c = Collatz{xn:13};
    for i in c.take(10) {
        println!("{}", i);
    }

    println!("XXXXXXXXXXX");

    let p = Prime{init:1};
    for pn in p.take(20) {
        println!("{}", pn);
    }

    println!("XXXXXXXXXXX");
    let res = primefact_v1(4857945);
    for factor in res {
        println!("{}", factor);
    }
}
