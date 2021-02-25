fn main() {
  
    let v = vec!(1,2,3,4,5,6,7,8,9);

    for (pos, elem) in v.iter().enumerate() {
        println!("{}: {}", pos, elem);
    }

    println!("Length: {}", v.iter().count());

    let iter1 = v.iter().step_by(2);
    for e in iter1 {
        println!("{}", e);
    }

    let iter2 = v.iter().map(|x| 2*x);
    for e in iter2 {
        println!("{}", e);
    }
    

    
    /*
    let t = 4;
    let iter2 = v.into_iter().filter(|x| x > &t);

    for e in iter2 {
        println!("{}", e);
    }
     */
    
}
