use std::usize;
use lib::{
    get_input, 
    get_vec_fr_string, 
    parse_and_execute,
    location::{
        init_locations,
        Location
    }, 
    objects::{
        init_objects, 
        Object
    }, 
};


fn main() {
    println!("Welcome to Lord of Rust Adventure.");
    let mut game_status = true;
    let mut location_of_player: usize = 0;
    let mut input = get_vec_fr_string(&"look around");

    let locations:Vec<Location> = init_locations();
    let objects:Vec<Object> = init_objects(&locations);

    while game_status{
        (game_status, location_of_player) = parse_and_execute(
            &input,
            &locations,
            &location_of_player,
            &objects
        );
        if game_status == true {
            input = get_input();
        }
    };

    println!("Bye.");
}



