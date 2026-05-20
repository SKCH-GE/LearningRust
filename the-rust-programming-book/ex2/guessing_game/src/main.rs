use std::io;


fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    
    // let creates a variable, by default immutable
    let mut guess = String::new();
    let x = 5;
    let y = 10;

    io::stdin() .read_line(&mut guess)
                // Chapter 4 will better explain how references "&" work
                .expect("Failed to read line");
                // this applies to read_line like "io::stdin().read_line(&mut guess).expect("Failed to read line");"
    
    println!("You guessed {}", guess);
    println!("x = {} and y = {}", x, y);

}