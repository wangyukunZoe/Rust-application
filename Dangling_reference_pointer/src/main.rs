fn main() {
    let r = dangle();
}

// s返回值在最后一个括号后已经被释放了，
// 指针指向了一个被释放的变量内存区域，这里就产生了一个悬空指针
//rust 在编译的时候就会报错，防止生成悬垂指针（引用）
/*
missing lifetime specifier
this function's return type contains a borrowed value, but there is no value for it to be borrowed from
 */
fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}
