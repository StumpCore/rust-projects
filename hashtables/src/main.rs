use std::{collections::HashMap, hash::Hash};

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blau"), 10);
    scores.insert(String::from("Gelb"), 50);
    
    let team_name = String::from("Blau");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_name}: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Lieblingsfarbe");
    let field_value = String::from("Blau");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    scores.entry(String::from("Gelb")).or_insert(50);
    scores.entry(String::from("Blau")).or_insert(50);
    println!("{scores:?}");

    let text = "Hallo Welt wunderbare Welt";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

}
