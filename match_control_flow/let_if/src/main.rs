fn main() {
    let v = Some(0u8);

    //使用 if let 就意味着放弃者穷举的可能
    match v {
        Some(3) => println!("three"),
        _ => println!("others"),
    }

    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}


/*

fn main() {
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        Some(4) => println!("four"),
        Some(5) => println!("five"),
        Some(6) => println!("six"),
        _ => (),
    }
}


*/