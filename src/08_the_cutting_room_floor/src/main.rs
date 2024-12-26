

fn main() {
    let prompt = &mut "The elevator is on the third floor, three rooms down the hallway next to a trio of cheeses.";
    let three = &mut "";
    loop {
        (*three, *prompt) = detect_three(prompt);
        if *three == "" { break }
        println!("Detected three: {three}");
    }

}

fn detect_three(text: &str) -> (&str, &str) {
    let length = text.len();
    let threes = ["third", "three", "trio"];
    
    for offset in 0..length {
        for option in threes.iter() {
            if length - offset < option.len() { continue }
            if text[offset..offset+option.len()] == **option {
                return (option, &text[offset+option.len()..]);
            }
        }
    }
    return ("", "")
}