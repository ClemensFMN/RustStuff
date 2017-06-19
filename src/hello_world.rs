/// Basic, getting started file

/// some stupid function
fn func1(x: i32) -> i32 {
	2*x
}

/// first shot at an recursive version of gcd calculation
fn mygcd(a: i32, b:i32) -> i32 {
   if b==0 {a}
   else {mygcd(b, a % b)}
}

/// recursive gcd with counting the number of iterations
fn mygcd_count(a: i32, b: i32, cnt: i32) -> (i32, i32) {
   if b==0 {(a, cnt+1)}
   else {mygcd_count(b, a % b, cnt+1)}   
}

/// iterative version counting the number of iteations
fn mygcd_count_v2(a:i32, b:i32) -> (i32, i32) {
    let mut u = a;
    let mut v = b;
    let mut cnt = 0;
    while v!=0 {
        let temp = u;
        u = v;
        v = temp % v;
        println!("u = {}, v = {}", u, v);
        cnt = cnt + 1;
    }
    (u, cnt)
}

enum SomeEnum {
    Typ1(i32),
    Typ2(i32, i32)
}


pub fn run_me() {

   println!("Hello, world!");
   let x = 5;

   println!("Value: {}, Function value: {}", x, func1(x));

   for x in 0..10 {
      println!("{}", x); // x: i32
   }

   let v = vec![3,5,11,9];

   for val in v {
       println!("Value: {}", val)
   }

    println!("GCD: {}", mygcd(13423,33432));

    let (gcd, cnt) = mygcd_count(128,24, 0);
    println!("gcd: {}, cnt: {}", gcd, cnt);

    let (gcd2, cnt2) = mygcd_count_v2(128, 24);
    println!("gcd: {}, cnt: {}", gcd2, cnt2);


   // Floating Point stuff
   let xf = 2.0_f32;
   let yf = xf.sqrt();
   println!("Value: {}", yf);

   // Fibonacci Numbers
   for n in  0..20 {
       let term1 = ((1.0_f32+5.0_f32.sqrt())/2_f32).powi(n);
       let term2 = ((1.0_f32-5.0_f32.sqrt())/2_f32).powi(n);
       let fib = 1_f32/5_f32.sqrt() *(term1 - term2);
       println!("Fib({}) = {}", n, fib);
   }

   let t1 = SomeEnum::Typ1(45);
   let t2 = SomeEnum::Typ2(4,5);
   

    let res = match t1 {
        SomeEnum::Typ1(a) => a,
        SomeEnum::Typ2(a,b) => a+b
    };

    println!("Value: {}", res);
    
}
