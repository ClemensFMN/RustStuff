extern crate rand;
use std::io;
use std::cmp::Ordering;
use tutorial::rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Number: {}", secret_number);

    let mut num_tries = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        num_tries = num_tries + 1;
        
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win! Number of tries: {}", num_tries);
                break;
            }
        }
    }
}
