use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let dice_roll;
    let maximum_dice_roll = 6;
    let high_roll = 4;
    
    loop {
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
        
    }
}
