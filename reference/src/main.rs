fn main() {
    let mut x = 10;

    let r1 = &x;
    let r2 = &x;

    println!("{}, {}, {}", x, r1, r2);

    x += 10;

    println!("{}", x);
}
