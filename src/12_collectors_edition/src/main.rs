fn main() {
    // Yes... I know, floating your currency is dangerous.
    let mut prices = vec![22.5, 29.1, 0.9, 100.25];
    
    println!("Original prices: {:?}", prices);
    save_twenty_five_percent(&mut prices);
    println!("Sale prices: {:?}", prices);
    println!("Fourth item is now: ${}", prices[3]);
    println!("No rainchecks. While stocks last. Terms and conditions apply. See in store for details.");
}

// This originally caused some confusion when I had:
// fn save_twenty_five_percent(mut items: &Vec<f64> ) {
// This apparently means something very different.
// The thing being *pointed to* can change, but not the contents.
// See: https://stackoverflow.com/questions/28587698/whats-the-difference-between-placing-mut-before-a-variable-name-and-after-the
fn save_twenty_five_percent(items: &mut Vec<f64> ) {
    for item in items {
        *item *= 0.75;
    }
1}