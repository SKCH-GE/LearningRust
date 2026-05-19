fn print_number(num: Option<i32>) {
    match num {
        Some(n) => println!("I see {}!", n),
        None    => println!("I see nothing!"), 
    }
}

fn main() {
    let x = Some(42);
    let y = None;

    print_number(x);
    print_number(y);
}