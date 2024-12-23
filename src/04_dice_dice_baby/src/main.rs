use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut dice_roll;
    let maximum_dice_roll = 6;
    let high_roll = 4;
    let run_again_response = 1;
    
    loop {
        let mut response = String::new();
        dice_roll = rand::rng()
            .random_range(1..=maximum_dice_roll);
        println!("You rolled: {dice_roll}");
        
        // I wish I knew more about using functions...
        // This is definitely a SRP violation.
        match dice_roll.cmp(&high_roll) {
            Ordering::Greater => println!("This is a high roll.😁"),
            Ordering::Equal => println!("This is a decent roll.😃"),
            Ordering::Less => println!("Eh. Good enough. 😐"),
        };
        
        println!("Select an option.\n(0) roll again\n(1) Finish.");
        io::stdin()
            .read_line(&mut response)
            .expect("Error reading line.");
        
        let response: u32 = match response
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("This is not a number. Please proceed to feel shame.");
                    println!("Here's another NUMBER while you wait.");
                    continue;
                }
            };

        match response.cmp(&run_again_response) {
            Ordering::Greater => println!("Invalid response. You'll get another number now.😑"),
            Ordering::Equal => break,
            Ordering::Less => println!("Okay, coming right up!😁"),
        }
    }
    println!("Done!");
}
