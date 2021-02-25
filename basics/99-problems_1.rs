fn prob_1(vec: &Vec<i32>) -> Option<i32> {
    let ind_last = vec.len();
    match ind_last {
        0 => None,
        _ => Some(vec[ind_last-1]),
    }
}


fn prob_6(vec1: &Vec<i32>, vec2: &Vec<i32>) -> bool {
    if vec1.len() != vec2.len() {return false} // if the vectors have different length we can return false right away
    let zpped = vec1.iter().zip(vec2.iter());
    for e in zpped { // otherwise we compare the vectors element-wise
        // println!("{:?}", e);
        if e.0 != e.1 {return false} // different -> false
    }
    return true;
}

// return true / false when vector contains value
fn prob_search_1(vec: &Vec<i32>, val: i32) -> bool {
    for &e in vec {
        if e == val {return true}
    }
    return false
}

// return index position of first match
fn prob_search_2(vec: &Vec<i32>, val: i32) -> Option<usize> {
    for (pos, &e) in vec.iter().enumerate() {
        if e == val {return Some(pos)}
    }
    return None;
}

// 
fn prob_search_3(vec: &Vec<i32>, val: i32) -> Vec<usize> {
    let mut positions: Vec<usize> = Vec::new();
    for (pos, &e) in vec.iter().enumerate() {
        if e == val {positions.push(pos)}
    }
    return positions;
}





// find last element of a list
fn prob_1_test() {
    let v1 = vec![1,2,3];
    let res1 = prob_1(&v1);
    println!("res1 = {:?}", res1);

    let v2 = vec![];
    let res2 = prob_1(&v2);
    println!("res1 = {:?}", res2);
}


// not really problem 6, but check whether two vecs have the same elements
fn prob_6_test() {
    let v1 = vec![1,2,3];
    let v2 = vec![1,2,3];
    let v3 = vec![1,2];
    let v4 = vec![1,2,6];

    println!("v1, v2 = {}", prob_6(&v1, &v2));
    println!("v1, v3 = {}", prob_6(&v1, &v3));
    println!("v1, v4 = {}", prob_6(&v1, &v4));
}

fn prob_search() {
    let v = vec![1,2,3,4,5,2,3,7,8,10];
    println!("prob_search_1: {:?}", prob_search_1(&v, 3));
    println!("prob_search_1: {:?}", prob_search_1(&v, -1));

    println!("prob_search_2: {:?}", prob_search_2(&v, 3));
    println!("prob_search_2: {:?}", prob_search_2(&v, -1));

    println!("prob_search_3: {:?}", prob_search_3(&v, 3));
    println!("prob_search_3: {:?}", prob_search_3(&v, -1));
}
    

fn main() {
    prob_1_test();
    prob_6_test();
    prob_search();
}
