enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

struct IpAddr{
    kind: IpAddrKind,
    address: string,
}

let home = IpAddr::V4(127,0,0,1);
let loopback = IpAddr::V6(Stirng::from("::1"));

fn main() {
}
