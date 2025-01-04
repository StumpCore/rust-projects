use crate::objects;

#[derive(Clone, PartialEq)]
pub struct Location {
    pub description: String,
    pub tag: String,
}


pub fn init_locations() -> Vec<Location>{
    vec![
        Location{
            description: "an open field".to_string(), 
            tag: "field".to_string()
        },
        Location{
            description: "a little cave".to_string(), 
            tag: "cave".to_string()
        },
    ]
}

pub fn execute_look(
    noun: &Option<String>,
    locations: &Vec<Location>,
    location_of_player:  &usize,
    objects: &Vec<objects::Object>
) {

    match noun.as_ref() {
        Some(x) if x.trim() == "around" => {
            println!("You are in {}.", locations[*location_of_player].description);
            objects::list_all_objects(objects, &locations[*location_of_player]);
        },
        Some(_x) => {
            println!("I don't understand what you want to see.");
        },
        None => {
            println!("I don't understand what you want to see.");
        }

    }
}


pub fn execute_go(
    noun: &Option<String>,
    locations: &Vec<Location>,
    location_of_player:  &usize,
    objects: &Vec<objects::Object>
) -> usize {

    let mut new_loc_of_player = *location_of_player;
    let init_noun = Some("around".to_string());

    for loc in 0..locations.len() {
        match noun.as_ref() {
            Some(x) => {
                if *x == locations[loc].tag{
                    if loc == *location_of_player {
                        println!("You can't get much closer than this.");
                    } else {
                        println!("OK.");
                        new_loc_of_player = loc;
                        execute_look(
                            &init_noun,
                            &locations,
                            &new_loc_of_player,
                            &objects
                        );
                    }
                }
            },
            None => {
                println!("Where to go?");
            }
        }
    }

    new_loc_of_player

}
