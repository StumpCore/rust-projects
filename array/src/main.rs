
fn main() {
    let v: Vec<f64> = vec![0.0,0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0,0.707, 1.0, 0.707];


    print(&v);
    print(&a);
}

fn print(n: &[f64]) {
    for elt in n{
        println!("{}", elt);
    }
}
