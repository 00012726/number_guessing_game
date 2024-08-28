use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("This is a number guessing game!");

    let secret_num: u32 = rand::thread_rng().gen_range(1, 101);
    let mut counter: u8 = 1;
    // println!("The secret number is {}", secret_num);
    
    loop {
        println!("Please input your guess: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                counter += 1;
                continue;
            }
        };
        
        match guess.cmp(&secret_num)
        {
            Ordering::Less => {
                println!("{}", "Too small!".red());
                counter += 1;
            }
            
            Ordering::Equal => {
                println!("{}", "You won!".green());
                println!("You tried {} times", counter);
                break;
            },
            Ordering::Greater => {
                println!("{}", "Too big!".red()); 
                counter += 1;
            }
        }
    }

    
}
