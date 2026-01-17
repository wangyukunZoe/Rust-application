fn main() {
    // s 不可用
    let s = "hello";    // s 可用
                             // 可以对 s 进行相关操作
    // String 比那些基础标量数据类型更复杂
    // 字符串字面值：程序里手写的那些字符串值，它们是不可变的

    //Rust 还有第二种字符串类型:String
    // - 在 heap 上分配。能够存储在编译时🧬未知数量的文本

    //可以使用from 函数从字符串字面值创建出 String 类型

    let mut s = String::from("Hello");

    s.push_str(", World!");

    println!("{}", s);

    //String 的 move 操作
    let s1 = String::from("Hello");
    let s2 = s1;    //注意这里的s1已经失效，因为内存指针已经指向s2，s1在变量作用域内已失效

   // println!("{}", s1); //s1此时将不可以被打印
    //error: could not compile `ownering_scope` (bin "ownering_scope") due to 1 previous error; 2 warnings emitted

    //带有clone克隆的 move 操作 （针对 heap）上的数据 -> 但这个方法有点吃内存
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 -> {}, s2 -> {}", s1, s2);

    // 如果只想在 stack 上进行复制 copy 复制即可
    //注意⚠️：基本类型都在 stack 上，引用类型都在 heap 上
    let x = 5;
    let y = x;

    println!("x -> {}, y -> {}", x, y);

    //stack 上的数据复制：copy trait
    /*
        拥有 copy - trait的类型
        所有整数类型
        boolean
        char
        所有浮点类型，例如 f64
        tuple元组 如果其所有的字段都是 copy的(i32,i32) 是 (i32, String) 不是
     */

}   // s 作用域到此结束，s不再可用
    
    /*
       当变量走出作用域时，Rust 会调用 drop 函数
       为了处理内存溢出问题，Rust采取了不同于 GC语言的方式- 对于某个值来说，当它拥有它的变量走出作用范围时，内存会立即自动
       的交换给操作系统
        - 变量和数据交互的方式：移动 move
            多个变量可以与同一个数据使用一种独特的方式来交互
            let x = 5;
            let y = x;
            整数是已知且固定大小的简单的值，这两个 5 被压到了 stack 中

        // 注意这里不仅是浅拷贝和深拷贝

        + 如果真的想对 heap 上面的 String 数据进行深度拷贝，而不仅仅是stack 上的数据，
        可以使用克隆clone 方法
     */