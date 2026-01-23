
fn main() {
    let five = Some(5); // 6
    let six = plus_one(five);   // 7
    let none = plus_one(None);

    println!("use Some five = {:?}", five);
    println!("use plus_one = {:?}", six);
    println!("None = {:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // 必须穷举所有情况，必须包含哪些没有涉及到的值
        None => None,
        Some(i) => Some(i + 1),
        //当然，如果实在需要涉及到没有的值的话，必须用_下划线通配符来替代其余没有列出的值
        
    }
}