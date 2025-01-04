fn main() {
  let mut composers = Vec::new();
  composers.push(Person {
    name: Some("Palestrina".to_string()),
    birth: 1525
  });

  let first_name = composers[0].name.take();
  let birth= composers[0].birth;

  println!("{}: {}", first_name.unwrap(), birth);
  
}

struct Person {
  name: Option<String>,
  birth: i32
}
