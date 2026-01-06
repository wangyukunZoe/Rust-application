fn main() {

    //call the function
    //使用创建的函数
    let z:i32 = my_function(22);
    println!("my_function returned: z: {}", z);
}

//创建第一个我们自己的函数
// fn my_function(x: i32) -> i32 这里的“->“ 是指函数返回值的类型
fn my_function(x: i32) -> i32 {
    //return 5;   //如果需要提前返回，可以使用 return 关键字
    println!("my_function called with:{}", x);
    let y:i32 = 10;
    y   // 这里创建的y变量是用来计算返回值的，是表达式的一种，因此不需要; 等价于return y;
    //return y;
}