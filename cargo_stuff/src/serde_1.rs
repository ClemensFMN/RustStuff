extern crate serde;
extern crate serde_json;


#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rect{
	ll: Point,
	ur: Point,
	dta: Vec<i32>,
}


pub fn run_me() {
    let p1 = Point{x:10, y:5};
    let serialized = serde_json::to_string(&p1).unwrap();
    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    let r1 = Rect{ll:Point{x:0,y:0}, ur:Point{x:4,y:9}, dta:vec![1,3,6]};
    let r1_ser = serde_json::to_string(&r1).unwrap();
    println!("serialized = {}", r1_ser);
}