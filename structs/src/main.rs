struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}

fn build_user(email:String, username:String) -> User {
    User { 
        active: true, 
        username,
        email, 
        sign_in_count: 1 
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        None => None, 
        Some(i) => Some(i+1)
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("first@email.com"), 
        String::from("usernameMartin")
    );

    let user2 = User{
        email: String::from("second@email.com"),
        ..user1
    };

    user1.email = String::from("another@email.com");
    println!("{}", user1.email);
    println!("{}", user2.email);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}", five.unwrap());
    println!("{}", six.unwrap());
    println!("{}", none.unwrap());
}
