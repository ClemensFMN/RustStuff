
// filter func taking a vector of data and a filter value
fn my_filter(data: Vec<i32>, filter_val: i32) -> Vec<i32> {
	data.into_iter()
		.filter(|e| e < &filter_val)	// i don't know why we need a & here!!
		.collect()
}

fn my_filter2(data: &Vec<i32>, filter_val: i32) -> Vec<i32> {
	let tmp = data.clone();
	tmp.into_iter()
		.filter(|e| e < &filter_val)
		.collect()
}


fn main() {
	// define a var outside the closure
	let y = 10;
	let cl1 = |x| {
		// y is captured inside the closure
		println!("x Value: {}, y value: {}", x, y);
	};
	cl1(2);

	let d = vec![1,2,3,4,5,6,7,8,9,10];
	let res = my_filter(d, 6);
	// btw, this does not work -> d has moved into the function...
	//println!("{:?}", d);
	println!("res: {:?}", res);

	let d2 = vec![1,2,3,4,5,6,7,8,9,10];
	// here we boorow the vector to the function and can use it afterwards...
	let res2 = my_filter2(&d2, 6);
	println!("{:?}", d2);
	println!("{:?}", res2);
	
}