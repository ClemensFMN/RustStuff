#[derive(Debug)]
struct Player {
    name: String,
    health: i32,
    score: u32
}


fn increase_health(p:&mut Player, increase:i32) {
    p.health += increase;
}


pub fn run_me() {
    let p1 = Player{name:String::from("Clemens"), health:10, score:3};
    println!("p1: {:?}", p1);
    println!("p1.health: {}", p1.health);
    let Player{health:h, ..} = p1;
    println!("{}", h);

    let mut p2 = Player{name:String::from("Susi"), health:10, score:3};
    println!("p2: {:?}", p2);
    increase_health(&mut p2, 5);
    println!("p2: {:?}", p2);
}
