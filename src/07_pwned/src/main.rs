use std::io;


fn main() {
    let mut question = String::from("What is your name?");
    prompt_for_input_with(&mut question);
    println!("Got: {question}");
}

fn prompt_for_input_with(question: &mut String) {
    let mut input = String::new();
    println!("{question}");

    io::stdin()
        .read_line(&mut input)
        .expect("Reading input failed.");
    question.push_str(&input);
}

