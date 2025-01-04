use crate::location::Location;

#[derive(Clone)]
pub struct Object <'a>{
    pub description: String,
    pub tag: String,
    pub loc: Option<&'a Location>
}

pub fn init_objects(locations: &Vec<Location>) -> Vec<Object>{
    let mut interim_obj = vec![
        Object{
            description: "a silver coin".to_string(), 
            tag: "silver".to_string(),
            loc: Some(&locations[0])
        },
        Object{
            description: "a gold coin".to_string(), 
            tag: "gold".to_string(),
            loc: Some(&locations[1])
        },
        Object{
            description: "a burly guard".to_string(), 
            tag: "guard".to_string(),
            loc: Some(&locations[0])
        },
        Object{
            description: "yourself".to_string(), 
            tag: "yourself".to_string(),
            loc: Some(&locations[0])
        },
    ];

    for i in 0..locations.len() {
        interim_obj.push(
            Object{
                description: locations[i].description.to_string(), 
                tag: locations[i].tag.to_string(),
                loc: None
            }
        );
    };

    interim_obj
}

pub fn list_all_objects(objects: &Vec<Object>, player_loc: &Location) {
    for obj in objects{
        if obj.description != "yourself"  {
            match obj.loc.as_ref() {
                Some(x) if *x == player_loc => {
                    println!("You see: {}", obj.description);
                },
                Some(_x) => {},
                None => {}
            }
        }
    }
}

fn object_has_tag(obj: &Object, noun: &str) -> bool {
    println!("{} -- {} -- {}", 
        obj.description,
        obj.tag,
        noun
    );

    if noun == obj.tag {
        return true
    }

    return false
}

pub fn get_object <'a>(noun: &str, list_of_obj: &'a Vec<Object<'a>>) -> Object<'a>{
    let obj = Object{
        description: "".to_string(), 
        tag: "".to_string(),
        loc: None 
    };

    for li_obj in list_of_obj {
        if object_has_tag(li_obj, noun) {
            return li_obj.clone();
        }
    }
    obj
}

pub fn get_visible<'a>(
    command: &str, 
    noun: &str,
    list_of_obj: &'a Vec<Object<'a>>
    ) -> Object<'a>{
        let obj = get_object(noun, list_of_obj);

        if obj.description == "".to_string() {
            println!("I don't understand {}.", command);
        } else {
            println!("You don't see any {} here.", noun);
        }

        return obj;
}

