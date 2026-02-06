fn main() {
    // {
    //     let r; //  ‘a
    //     //  在 Rust 中，必须给变量赋予初始值
    //     // let b = r;
    //     {
    //         let x = 5; //  -+-- 'b
    //         r = &x; //此时发生了悬垂指针错误❎，也成为迷途指针  // -+
    //         //  borrowed value does not live long enough
    //         //Rust 内置了借用检查器

    //         //⚠️ 因为此时r变量指向的x引用在同一块作用域中，下面这行代码是可以被执行的，不会发生错误
    //         // println!("r: {}", r);
    //     }
    //     // 但随着离开了x变量的作用域，r指针不知道要指向哪里，此时发生了严重的悬垂指针错误，也是野指针
    //     println!("r: {}", r);
    // }

    let x = 5;
    let r = &x;

    println!("r:{}", r);
}
