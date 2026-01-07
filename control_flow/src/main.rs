fn main() {
    let a:i32 = 5;

    if a > 5 {
        println!("Bigger than 5");
    } else if a > 3 {
        println!("Bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    // 注意⚠️：这里控制流语句内的变量数据类型必须一致
    let _b: i32 = if a > 5 { 1 } else { -1 };

    //loop 循环♻️
    'outer: loop{
        println!("loop forever!");
        loop{
            break 'outer;
        }
    }

    // 直到x变量为 5 时结束循环，这种循环通常用于测试的时候很有用
    // let x:i32 = loop{
    //     break 5;
    // };

    // while loop
    let mut a: i32 = 0;

    while a < 5 {
        println!("a is {a}");
        a = a + 1;
    }

    //for loop
    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("{}", element);
    }

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {element}");
    // }

}
