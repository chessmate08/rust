use rand;
use std::{io, process::exit};

fn main() {
    let actual = rand::random_range(1..=100);
    println!("Write number:   ");
    let mut num: i32 = user_num();
    let mut guesses: i32=1;
    while num != actual {
        guesses+=1;
        if guesses==6 {println!("You lost, run again to play again\n\n"); exit(0);}

        let msg: &str= if num < actual {"Go higher"} else {"Go lower"};
        println!("{}\n\nWrite number:", msg);
        
        num = user_num();
        };
    println!("You got it in {guesses} guesses!")
}

fn user_num() -> i32 {
    
    let mut buffer: String = String::new();
    
    let _x=io::stdin().read_line(&mut buffer);
    
    let number: i32 = buffer.trim().parse::<i32>().unwrap();
    number
}