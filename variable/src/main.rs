const MAX_POINTS:u32 = 100_000; // 常量一定要大写

fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;  //如果想中途修改变量值，就必须使用mut关键字转换为⇄可变类型
    println!("The value of x is {}", x);

    //shadowing 
    let q = 5;
    let q = q + 1;
    let q = q * 2;

    println!("The value of x is {}", q);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("{}", spaces);
}
