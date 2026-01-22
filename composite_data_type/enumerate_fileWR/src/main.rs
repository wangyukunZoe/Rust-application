enum Message {
    Quit,   //这个变量什么都没有做，一样可以被声明成枚举类型
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 定义一个方法
impl Message {
    fn call(&self) {}
}

fn main() {

    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);

    m.call();
}
