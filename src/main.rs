use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Write your bet");

    let random_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut bet = String::new();

        io::stdin()
            .read_line(&mut bet)
            .expect("Error to read your bet!");

        let bet: u32 = match bet.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a number!");
                continue;
            }
        };

        println!("You said: {} ", bet);

        match bet.cmp(&random_number) {
            Ordering::Less => println!("Very low!"),
            Ordering::Greater => println!("Very high!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
