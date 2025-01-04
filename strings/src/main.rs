fn main() {
    let mut s = String::new();
    let data = "initialer Inhalt";

    let s = data.to_string();
    println!("{s}");

    let s = "initialer Inhalt".to_string();
    println!("{s}");

    let s = String::from("initialer Inhalt");

    println!("{s}");

    let s1 = String::from("Hallo ");
    let s2 = String::from("Welt!");
    println!("{s1}");
    println!("{s2}");

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

}
