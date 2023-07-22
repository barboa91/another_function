use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // original_main();
    f_to_c();
    //  ownership();


}

fn ownership() {
    let s ="hello";
}

fn original_main() {
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

fn f_to_c(){

    println!("Enter a temp in fahrenhite to convert it to celsius");

    let mut fah = String::new();

        loop{
            io::stdin()
                .read_line(&mut fah)
                .expect("Failed to read in");

            let  x: u32  = match fah.trim().parse(){
                Ok(fart) => fart,
                Err(e) => {{println!("{}",e)};continue},
            };

            let xCel = ((x as f32) -32.0)/1.6;
            
            // };
            println!("{x} Fahernheit is equal to {xCel}");  

            break;

        }


}