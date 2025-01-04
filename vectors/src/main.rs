fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(8);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("Das dritte Element ist {third}"),
        None => {
            println!("Es existiert kein drittes Element.");
            println!("Aktuell existieren nur:");
            for i in &v {
                print!("{i} ");
            }

            for i in &v {
                print!("{i} ");
            }
            
        },
    }
}
