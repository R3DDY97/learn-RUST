
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    
    println!("Guess the number!\n");


    let secret_number = rand::thread_rng().gen_range(1, 101);

   // println!("The secret number is {}", secret_number);

    loop {
        
            println!("\nInput your guess number ");

            let mut guess = String::new();

            io::stdin().read_line(&mut guess)
                .expect("Failed to load readline");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        

            println!("you have guessed {}", guess);
        

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo small...!!"),
            Ordering::Greater => println!("\nToo big...!!"),
            Ordering::Equal => {
                println!("\n\nYou Win...!!\n");
                break;
            }
        }
        
          
    }

    
}
