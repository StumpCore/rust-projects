use crate::garden::vegetables::Asparagus;
pub mod garden;

fn main() {
    let plant = Asparagus{};
    println!("Ich baue {plant:?} an!");
}
