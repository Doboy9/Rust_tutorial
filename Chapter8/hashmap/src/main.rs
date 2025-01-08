use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Yellow".to_string(), 10);
    scores.insert("Blue".to_string(), 5);

    let score = scores.get("Blue").copied().unwrap_or(0);
    println!("{score}");
    for (key, values) in &scores {
        println!("{key}: {values}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("{field_name}");

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // Using the entry method to only insert if the key does not already have a value
    scores.entry(String::from("Green")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
