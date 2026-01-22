enum IpAddresskind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddresskind::V4(127, 0, 0, 1);
    let loopback = IpAddresskind::V6(String::from("::1"));
    
}
