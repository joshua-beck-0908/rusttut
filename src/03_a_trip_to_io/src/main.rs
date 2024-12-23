use std::io;

fn main() {
    let mut name = String::new();
    
    println!("What shall we call you?");
    io::stdin()
        .read_line(&mut name)
        .expect("No named given.");
    
    // Hopefully this is still cool...
    println!("😱You are {name}y Mc{name} face!😱");
}
