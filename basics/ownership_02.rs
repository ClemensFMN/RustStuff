// ********************** NOT modifying stuff in a function **********************

// a function taking a vector, processing it somehow, but NOT modifying it
// we use a reference to the vec
fn use_ro_vec(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for el in vec {
        sum = sum + *el;    // in light of the issue in the next function, using *el is probalby more consistent...
    }
    sum
}

fn use_and_return_ro_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    // as we get a reference, it seems that a deref is required - i don't fully get this :-(
    for el in vec {
        res.push(*el);
    }
    res.push(34);
    res.push(4);
    res
}


fn use_ro_array(slice: &[i32]) -> i32 {
    let mut sum  = 0;
    for el in slice {
        sum = sum + el;
    }
    sum
}

fn use_and_modify_array(slice: &mut[i32]) {
    slice[0] = 10;
}

// this does not work because the array size must be known @ compile time
//fn use_and_return_ro_array(slice: &[i32]) -> [i32] {
//    let size = slice.len();
//    let res: [i32; size] = [0; size];
//}


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

fn main() {
    let v1 = vec![1,2,3];
    // we need to provide a ref to the vec
    println!("{}", use_ro_vec(&v1));

    let res = use_and_return_ro_vec(&v1);
    for el in res {
        print!("{} ", el);
    }

    println!("");

    let arr = [1,2,3,4,5];
    println!("Array sum: {}", use_ro_array(&arr));

    let mut arr2 = [1,2,3,4,5];

    use_and_modify_array(&mut arr2);
    
    for i in 0..arr2.len()-1 {
        println!("array element {}", arr2[i]);
    }
    
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
