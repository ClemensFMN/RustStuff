


fn main() {
    println!("Hello World");
    let buf1 = [3; 5];

    println!("Buffer Length: {}", buf1.len());

    for x in &buf1 {
        println!("{}", x);
    }
    // this does not work as the buf is R/O
    // buf1[0] = 0;
   
    println!("***********************");

    let mut buf2 = [3; 5];

    for x in &buf2 {
        println!("{}", x);
    }
    buf2[0] = 0;
    for x in &buf2 {
        println!("{}", x);
    }
}