use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("> Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut count = 1;
    loop {
        println!("> Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //println!("\n");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("> Too small!\n"),
            Ordering::Greater => println!("> Too big!\n"),
            Ordering::Equal => {
                println!("> You guessed the number in {} tries!", count);
                break;
            }
        }
        count+=1;
    }
}
