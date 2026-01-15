fn plus_five(x: i32) -> i32 {
    x + 5   //这里千万不能写分号，否则这里就是一个语句
}

fn five() -> i32 {
    5
}

fn main() {

    let x = plus_five(6);
    println!("The value of x is:{}", x);       //output: The value of x is:11

    let x = five();
    println!("The value of x is:{}", x);   //output: The value of x is:5
    
    println!("Hello, world!");
    another_function(5,6);    //argument

    //sentence 语句
    // let x = (let y = 6); //这个动作在 Rust 中是不可以的，后面不可以是一个语句
    let x = 5 + 6;  //表达式可以是一个语句

    let y = {
        let x = 1;
        x + 3;  //() 语句的返回值，但这个返回值是不可以被打印出来的
    };

}

//annotate
fn another_function(x: i32, y: i32) {   //parameter
    println!("Another function");
    println!("the value of x is: {}", x);
    println!("the value of x is: {}", y);

}

// 愿你的生命中有足够多的云鬓，去绘制一幅美丽的黄昏～