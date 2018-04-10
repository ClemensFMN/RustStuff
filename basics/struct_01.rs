#[derive(Debug)]
struct Point {
	x: u32,
	y: u32
}

impl Point {
	fn new() -> Point {
		Point{x:0, y:0}
	}

	fn build(x:u32, y:u32) -> Point {
		Point{x:x, y:y}
	}
}

#[derive(Debug)]
struct Rectangle {
	p1: Point,
	p2: Point
}

// this is a function which takes a struct as input
fn area(rect: &Rectangle) -> u32 {
	let deltax = rect.p2.x - rect.p1.x;
	let deltay = rect.p2.y - rect.p1.y;
	deltax*deltay
}

// the same thing but using a method
impl Rectangle {
	fn area_v2(&self) -> u32 {
		let deltax = self.p2.x - self.p1.x;
		let deltay = self.p2.y - self.p1.y;
		deltax*deltay
	}
}


fn main() {
	let p0 = Point::new();
	println!("{:?}", p0);
	let p02 = Point::build(1,2);
	println!("{:?}", p02);


	let p1 = Point {x:2, y:5};
	println!("{:?}",p1);
	println!("Point with coords: {},{}", p1.x, p1.y);

	let r1 = Rectangle{p1:Point{x:1,y:2}, p2:Point{x:10,y:5}};
	println!("Area: {}", area(&r1));
	// we need to only borrow the struct in the area function so that we still can use afterwards
	println!("rect1 is {:?}", r1);

	println!("Area, v2: {}", r1.area_v2());
}