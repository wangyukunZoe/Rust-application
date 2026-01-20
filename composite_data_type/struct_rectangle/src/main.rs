#[derive(Debug)]    //derive 是派生的意思
                    //这里是让 Rectangle 结构体派生于derive接口 trait
                    //dbg!

struct Rectangle{
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

//使用元组
fn area_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

//使用结构体
fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
fn main() {

    // use function
    let w = 30;
    let l = 50;

    println!("{}", area(w, l));

    // use tuple
    let rect = (30, 50);

    println!("{}", area_tuple(rect));

    // use struct
    let rect = Rectangle{
        width: 30,
        height: 50,
    };

    println!("{}", area_struct(&rect));

    //打印结构体
    //`Rectangle` doesn't implement `std::fmt::Display`
    //= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
    println!("{:#?}", rect); //这里需要通过 debug 模式显式的输出结构体里面的东西
                            // # 警号可以使struct 内的内容与结构易读
}
