fn main() {
    let mut s = String::from("Hello world");
    let wordIndex = first_world(&s);

    s.clear();  //但是，如果这个字符串被清楚🆑，wordIndex 
                //内的内容依旧不会发生变化，因为它所对应的函数已经完成了返回值
    println!("{}", wordIndex);
}

//找到字符串中空格所在的位置
fn first_world(s: &String) -> usize {
    //依次检查 String 字符数里面的每个字节
    let bytes = s.as_bytes();

    //用模式匹配，对元组进行结构
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '  {
            return i;
        }
    }
    s.len()
}