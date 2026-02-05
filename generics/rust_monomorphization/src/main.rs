//单态化
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Some(5);
    let float = Some(5.0);

    //单态化后处理的代码
    //let integer = Option_i32::Some(5);
    // let float = Option_f64::Some(5.0);
}
