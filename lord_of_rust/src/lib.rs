use std::io;
use std::io::Write;

pub mod location;
pub mod objects;

pub fn get_vec_fr_string(input: &str) -> Vec<Option<String>> {
    input
        .trim()
        .split_whitespace()
        .map(|word| {
            if word.is_empty(){
                None
            } else {
                Some(word.to_string())
            }
        })
        .collect()
}

pub fn get_input() -> Vec<Option<String>>{
    let mut input: String;
    let mut input_vec: Vec<Option<String>> = vec![];

    while input_vec.len() < 2 {
        print!("\n-->");
        io::stdout().flush().unwrap();

        input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read Line.");

        input_vec = get_vec_fr_string(&input);

        if input_vec[0] == Some("quit".to_string()) {
            input_vec.push(Some("quit2".to_string()));
            break;
        }

        if input_vec.len() < 2 {
            println!("What is that, fool?!");
            println!("Provide more direction.");
            println!("--> [verb] [noun]");
        }
    }
    input_vec
}

pub fn parse_and_execute(
    command: &Vec<Option<String>>,
    locations: &Vec<location::Location>,
    location_of_player: &usize,
    objects: &Vec<objects::Object>,
    ) -> (bool, usize) {

    let arg1 = &command[0];
    let new_loc_of_player;

    match arg1.as_ref() {
        Some(x) if x.trim() == "quit" => (false, 0),
        Some(x) if x.contains("look") => {
            location::execute_look(
                &command[1],
                locations,
                location_of_player,
                objects
            );
            (true, *location_of_player)
        }
        Some(x) if x.contains("go") => {
            new_loc_of_player = location::execute_go(
                &command[1],
                locations,
                location_of_player,
                objects
            );
            (true, new_loc_of_player)
        }
        Some(_x) => {
            println!("I don't know how to do {}", _x);
            (true, *location_of_player)
        }
        None => (true, *location_of_player),
    }
}
