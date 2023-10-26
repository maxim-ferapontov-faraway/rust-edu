use log::{debug, info, trace};
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    env_logger::init();

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    debug!("secret_number {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        info!("You gussed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                trace!("You win!");
                break;
            }
        }
    }
}
