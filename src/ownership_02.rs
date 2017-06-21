// ********************** NOT modifying stuff in a function **********************

// a function taking a vector, processing it somehow, but NOT modifying it
// we use a reference to the vec
fn use_ro_vec(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for el in vec {
        sum = sum + el;
    }
    sum
}

// define a stupid structure
struct simple_struct {
    val1: i32,
    val2: i32,
}

// a function using this structure, but NOT modifying it
fn use_ro_struct(s: &simple_struct) -> i32 {
    s.val1 + s.val2
}

// ********************** modifying stuff in a function **********************
// we pass a mutable reference
fn use_vec(vec: &mut Vec<i32>) {
    vec[0] = -1;
}

fn use_struct(s: &mut simple_struct) {
    s.val1 = s.val1 + s.val2;
}

pub fn run_me() {
    let v1 = vec![1,2,3];
    // we need to provide a ref to the vec
    println!("{}", use_ro_vec(&v1));

    let mys1 = simple_struct{val1:3, val2:10};
    println!("{}", use_ro_struct(&mys1));

    let mut v2 = vec![1,2,3];
    // the most notable difference is that we call the function and it does NOT return a value; instead the argument is modified
    use_vec(&mut v2);
    for el in v2 {
        print!("{} ", el);
    }

    println!("");

    
    let mut mys2 = simple_struct{val1:3, val2:10};
    use_struct(&mut mys2);
    println!("{}, {}", mys2.val1, mys2.val2);
    
}
