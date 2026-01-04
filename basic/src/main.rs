fn main() {
    // creation
    let a:i16 = 5;

    //mutability
    let mut b:i32 = 5;
    // let b:i32 = 5;
    // b = 10;  //will be marked a mistake
    // 在 rust 中，变量是不可变的，也就是说一般的 let 声明的变量是常量
    //如果想要替换变量内的值，需要 mut（static） 转换为静态变量后再操作

    //shadowing
    let c:i32 = 10;
    let c:i32 = 20; //当存在两个相同变量时，按照顺序取最后创建的那个变量 例如这里的c只变量的内容只有 20，而不是 10

    println!("c is {c}");

    //scope
    //作用域
    let d:i32 = 30;

    {
        // let d:i32 = 40;
        // println!("inner d is:{d}");

        let e:i32 = 40;
        println!("inner e is:{e}");
    }

    println!("d is {d}");

}
