use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
            vec![
                "many marigals".to_string(),
                "Tenebrae Responsoria".to_string(),
            ]);
    table.insert("Caravaggio".to_string(),
            vec![
                "The Musicians".to_string(),
                "The Calling of St. Matthew".to_string(),
            ]);
    table.insert("Cellini".to_string(),
            vec![
                "Perseus with head".to_string(),
                "A salt cellar".to_string(),
            ]);

    sort_works(&mut table);
    show(table);
}

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("   {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for(_artist, works) in table {
        works.sort();
    }
}
