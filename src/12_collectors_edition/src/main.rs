use std::collections::HashMap;

// Yes... I know, floating your currency is dangerous.
fn main() {
    let discount_25_percent = vec!["Coats", "Hats", "Sunglasses"];
    let discount_40_percent = vec!["Ribbons", "Lipstick"];
    // Apparently you don't need to specify the type here because it can be
    // statically determined. The IDE still shows the implied type.
    let mut discounted_items = HashMap::new();
    let mut pitches = Vec::new();

    for item in discount_25_percent {
        add_discounted_item(&mut discounted_items, item, 25.0);
    }
    
    for item in discount_40_percent {
        add_discounted_item(&mut discounted_items, item, 40.0);
    }

    pitches.push(get_discount_spiel(&discounted_items, "Coats", 22.50));
    pitches.push(get_discount_spiel(&discounted_items, "Ribbons", 12.30));
    pitches.push(get_discount_spiel(&discounted_items, "Chocolates", 4.55));
    
    for pitch in pitches {
        println!("{pitch}");
    }

}

// This was surprisingly confusing to get right....
// I had &str in the HashMap since I only intended to use quoted strings.
// But it kept on complaining about lifetimes, so I switched it to String.
// I'm still learning, don't judge me!
fn add_discounted_item(
    discount_list: &mut HashMap<String, f64>,
    category_name: &str,
    discount_percent: f64) {
    
    discount_list.insert(category_name.to_string(), discount_percent);
}

// TODO: Grammar...
fn get_discount_spiel(
    discount_list: &HashMap<String, f64>, 
    category_name: &str,
    original_price: f64) -> String {
    let discount = discount_list.get(category_name).copied().unwrap_or(0.0);
    
    let new_price = original_price * (discount / 100.0);

    if discount > 30.0 {
        return format!("This {category_name} was {original_price}, now only {new_price}. What an incredible bargin!");
    } else if discount > 0.0 {
        return format!("This {category_name} was {original_price}, but is now only {new_price}. What a deal!");
    } else {
        return format!("No sales on {category_name}, buy it now at {original_price} or check back next week!");
    }


}
