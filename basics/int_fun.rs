// trying out some func from https://doc.rust-lang.org/std/primitive.i32.html

fn main() {

    let s1 = i32::from_str_radix("ABG", 16);
    println!("{:#?}", s1);

    match s1 {
        Ok(v1) => println!("Parsed Value = {}", v1),
        Err(e) => println!("Error: {}", e)
    };

    let i1:i32 = 35;
    let ones_i1 = i1.count_ones();
    println!("Numer: {:b}, Numberof ones: {}", i1, ones_i1);

    let i2: i32 = 2;
    println!("{}", i2.pow(5));

}