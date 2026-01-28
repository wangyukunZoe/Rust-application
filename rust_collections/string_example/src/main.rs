fn main() {
    let mut s = String::from("foo");
    let s1 = String::from("bar");
    // s.push_str("bar");
    s.push_str(&s1);

    println!("{}", s);

    let mut s1 = String::from("lo");
    s1.push('l');   // 附加单个字符

    //拼接字符串
    let s2 = String::from("Hello, ");
    let s3 = String::from("World!");

    let s4 = s2 + &s3;  // s3中的内容是复制过来的！
    println!("{}", s4);
    // println!("{}", s2); // s2的所有权失效 -> 使用了类似于fn add(self, s:&str) -> String {} 的方法
    // println!("{}", s3); // 这里使用了解引用强制转换的技术deref coercion

    println!("=====================");
    let s5 = String::from("Tic");
    let s6 = String::from("Tac");
    let s7 = String::from("Toe");

    // let s8 = s5 + "-" + &s6 + "-" + &s3;
    // println!("{}", s8);

    //使用 format!()宏后的输出
    let finalS = format!("{}-{}-{}", s5, s6, s7);
    println!("{}", finalS); //format不会获得任何参数的所有权

}
