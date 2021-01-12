use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "AoC_2020_01_input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vector = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        //println!("{}. {}", index + 1, line);
        vector.push(line.parse::<i32>().unwrap());
    }
    //println!("{:?}", vector);

    // Part 1
    for val1 in vector.iter() {
        for val2 in vector.iter() {
            let sum = val1 + val2;
            //println!("{:?}, {:?} => {:?}", val1, val2, sum);
            if(sum == 2020) {
                println!("FOUND: {:?}, {:?}", val1, val2);
            }
        }
    }

    // Part 2
    for val1 in vector.iter() {
        for val2 in vector.iter() {
            for val3 in vector.iter() {
                let sum = val1 + val2 + val3;
                    //println!("{:?}, {:?} => {:?}", val1, val2, sum);
                    if(sum == 2020) {
                        println!("FOUND: {:?}, {:?}, {:?}", val1, val2, val3);
                    }
            }
        }
    }




}

