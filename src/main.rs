use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Hello, guess the number!");

    let secret_numb = rand::thread_rng().gen_range(1,  101);

    loop{
        let mut guess = String::new();

        println!("Input your guess");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read string");

        //let guess: u32 = guess.trim().parse().expect("Type a number!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_numb){
            Ordering::Less =>  println!("{}","Guess number is smaller than secret".red()),
            Ordering::Greater => println!("{}","Guess number is bigger than secret".red()),
            Ordering::Equal => {
                print!("{}", "You win!".green());
                println!(" secret number was {}",secret_numb);
                break;
            },
        }
    }
}
