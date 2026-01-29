fn main() {
    let s1 = String::from("hello");
    //let h = s1[0];  //这是不可以的 无法使用整数进行索引，不支持索引进行访问

    let len = String::from("Hola").len();   //输出 String 的长度

    //unicode 标量值
    println!("{}", len);    //4

    let len = String::from("Здравствуйте").len();   //输出 String 的长度
    println!("{}", len);    // 3 Unicode 标量值失效的情况   目前这个错误已经被修复！

    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    // 3: 208,151

    //已标量值的形式看待这个字符串
    let w = "संस्कृतावाक्";

    // for b in w.bytes() {
    //     println!("{}", b);
    // }

    for b in w.chars() {
        println!("{}", b);
    }
    
    //可以用字形簇得到完整的字形分解 charsetBreaker grapheme cluster

    // 对字符串创建一个切片
    let hello = "Здравствуйте";
    let s = &hello[0..4];   // 从索引 0 到索引 4    // 要谨慎使用。需要沿着字符边界进行切割

    println!("{}", s);  //这里的 0..3会导致错误和恐慌 
}
