fn main() {
    let v = 0u8;

    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        //下划线通配符来配对，这样可以不用通配符所有的类型了
        _=> (),
    }
}
