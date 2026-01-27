fn main() {
    let v = vec![1, 2, 3, 4, 5];    //vector 默认索引从 0 开始
    let third: &i32 = &v[2];

    // let third: &i32 = &v[100];
    //thread 'main' (327893) panicked at src/main.rs:3:25:
    // index out of bounds: the len is 5 but the index is 100
    
    println!("The third element is {}", third);

    // 这种方式不会出现崩溃和恐慌
    match v.get(100) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
}
