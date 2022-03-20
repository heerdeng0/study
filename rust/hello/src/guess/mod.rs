extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn main()
{
    let secret_num = rand::thread_rng().gen_range(1, 10);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
    
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}