use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // original_main();
    // f_to_c();
    //  ownership();
    // fibo(10);
    randomtesting();



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

fn fibo(n_fib : u32){
    let mut fib = String::new();

    println!("Get the nth number in fibo sequence");
    

    io::stdin()
        .read_line(&mut fib)
        .expect("didnt get the read");



    println!("{n_fib}");
}

fn get_input() -> String{

    let mut inp = String::new();

    io::stdin()
    .read_line(&mut inp)
    .expect("didnt get the read");


    return inp;

}

fn randomtesting() {

    let mut s = String::from("hello world");
    println!("{} <--, this is s",s);


    let word = first_word(&s); // word will get the value 5
    println!("{} <--, this is word",word);

    s = "farts".to_string();
    // s.clear(); // this empties the String, making it equal to ""
    println!("clearing s");

    println!("{} <--, this is s",s);
    // println!("{} <--, this is word",word);

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!


}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}