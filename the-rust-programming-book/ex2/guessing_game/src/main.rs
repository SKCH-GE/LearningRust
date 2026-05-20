use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    
    // let creates a variable, by default immutable
    let mut guess = String::new();
    let secret_number= rand::thread_rng().gen_range(1,101);
    
    io::stdin() .read_line(&mut guess)
                // Chapter 4 will better explain how references "&" work
                .expect("Failed to read line");
                // this applies to read_line like "io::stdin().read_line(&mut guess).expect("Failed to read line");"

    let guess:u32 = guess.trim().parse()
                        .expect("Please type a number");
    
    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!"),
    }

    println!("You guessed {}", guess);
    println!("the secret number: {}", secret_number);
}