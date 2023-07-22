use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    println!("The number is {}!", secret_number);
    
    loop{
        println!("enter your guess");

        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You entered {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Smoll"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {println!("Jussstt Right");break;}
        }
    }

    
}