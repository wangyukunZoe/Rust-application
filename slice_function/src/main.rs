fn main() {
    let my_string = String::from("Hello world");
    let wordIndex1 = first_world(&my_string[..]);

    let my_string_literal = "hello world";
    let wordIndex2 = first_world(my_string_literal);

    //s.clear();  //但是，如果这个字符串被清楚🆑，wordIndex 
                //内的内容依旧不会发生变化，因为它所对应的函数已经完成了返回值
                // 如果使用了利用字符串切片技术的函数，就会在编译🧬时报错，防止生成错误的结果
    
    println!("{}", wordIndex1);
    println!("{}", wordIndex2);

}

//利用字符串切片技术来做
fn first_world(s: &str) -> &str {
    //依次检查 String 字符数里面的每个字节
    let bytes = s.as_bytes();

    //用模式匹配，对元组进行结构
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '  {
            return &s[..i];
        }
    }
    &s[..]
}

// //找到字符串中空格所在的位置
// fn first_world(s: &String) -> usize {
//     //依次检查 String 字符数里面的每个字节
//     let bytes = s.as_bytes();

//     //用模式匹配，对元组进行结构
//     for(i, &item) in bytes.iter().enumerate(){
//         if item == b' '  {
//             return i;
//         }
//     }
//     s.len()
// }