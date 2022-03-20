//use std::{thread, time};

//use std::io::Error;

mod window;
mod guess;

fn main()
{
    /* println!("Hello, world!");

    let ten_secs = time::Duration::from_secs(10);

    thread::sleep(ten_secs); */

    guess::main();

    window::main();
}

