#![allow(dead_code)]

enum Message {
    Quit,
    Move{x:i32, y:i32},
    Wr{s:String},
}

fn prs_msg(m:Message) {
    match m {
        Message::Quit => println!("Quit"),
        Message::Move{x,y} => println!("Move: {},{}", x, y),
        Message::Wr{s} => println!("Wr: {}", s),
    }
}

pub fn run_me() {
    println!("Struct 2");
    let m1 = Message::Quit;
    let m2 = Message::Move{x:3,y:4};
    let m3 = Message::Wr{s:String::from("Hello")};

    prs_msg(m1);
    prs_msg(m2);
    prs_msg(m3);
}
