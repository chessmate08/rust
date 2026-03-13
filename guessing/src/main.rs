use rand;
use std::io;

fn main() {
    let mut actual = rand::random_range(1..=100);
    
    loop {
        println!("Write number 1 - 100 inclusively:   ");
        let num: u8 = loop {
            let mut buffer = String::new();
            let input = io::stdin().read_line(&mut buffer);
            
            match input {
                Result::Err(error) => {
                    println!("Error getting value: {:?}", error);
                    continue;
                },
                Result::Ok(_val) => ()
            }
            match buffer.trim().parse::<u8>() {
                Ok(val) => {
                    break val;
                },
                Err(e) => {
                    println!("Not a positive integer or less than 256: {:?}", e);
                    continue;
                }
            }
        };
        if num > actual {
            println!("{num} is too high\n");
        } else if num < actual {
            println!("{num} is too low\n");
        } else {
            println!("{num} is correct!");
            let mut buffer = String::new();
            let _x = io::stdin().read_line(&mut buffer);
            println!("play again?");
            if buffer.trim().len() >= 1 && buffer.trim().chars().next().unwrap().to_lowercase().next().unwrap() == 'y' {
                actual = rand::random_range(1..=100);
                continue;
            }
            break;

        }
    }

}