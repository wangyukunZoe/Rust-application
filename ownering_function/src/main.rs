
fn main() {
    // let s = String::from("Hello world!");

    // take_ownership(s);

    // let x = 5;

    // makes_copy(x);

    // println!("x: {}",  x);

    println!("=====================================");
    // 返回值所有权的移动
    let s1 = give_ownership();

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2);  //s2这里就访问不了，因为已经离开作用域丢出去了

    println!("{} {}", s1, s3);

    println!("======================================");

    let s1 = String::from("hello");

    let(s2, len) = calculate_length(s1);    //这里的s1被自动销毁 除非加上s1.clone()
    //  let(s2, len) = calculate_length(s1.clone()); 

    println!("The length of '{}' is {}.", s2, len);

    //    println!("The length of '{}' is {}. {}", s2, len, s1);

}

fn calculate_length(s:String) -> (String,usize) {
    let length = s.len();

    (s, length) // length 返回一个副本即可
}
 //这里实际上可以使用引用技术来进行简化


fn give_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}


//这里穿进去的值都是复制的值，切记！并非引用
// fn take_ownership(some_string: String){
//     println!("{}", some_string);
// }

// fn makes_copy(some_number: i32){
//     println!("{}", some_number);
// }