fn main() {
    let v = vec![100, 32, 57];

    //可变的 vector
    let mut v1 = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    // 动态更新 vector
    for i in &mut v1 {
        *i += 50;
        println!("{}", i);
    }

    for i in v1 {
        println!("{}", i);
    }
}
