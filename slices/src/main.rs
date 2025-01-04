fn main() {
    let s = String::from("Hallo Welt");
    let slice = &s[..2];
    let slice2 = &s[2..4];

    println!("{slice} {slice2}");

    let fword = first_word(&s);
    println!("{fword}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
