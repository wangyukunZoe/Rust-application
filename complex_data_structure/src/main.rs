fn main() {
    //创建元组 tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //利用模式匹配来解释刚刚创建的元组值
    let (x, y, z) = tup;

    println!("{}, {}, {}, ", tup.0, tup.1, tup.2);
    println!("{}, {}, {}, ", x, y, z);

    //数组也可以将多个值放到一个数组里面
    //声明一个数组
    let a = [1, 2, 3, 4, 5];
    //如果你想让你的数据存放在 stack 上而不是 heap 上，或者想保证固定数量的元素，数组是不二之选
    //但是数组没有 vector 灵活，vector 由标准库提供

    
    // Fixed-size array of string slices (12 elements)
    let months: [&str; 12] = [
        "January", 
        "February", 
        "March", 
        "April",
        "May", 
        "June", 
        "July", 
        "August",
        "September",
         "October", 
         "November", 
         "December",
    ];

    // Example usage: print each month with its number
    for (i, &month) in months.iter().enumerate() {
        println!("{}: {}", i + 1, month);
    }

    let first = months[0];
    let second = months[1];
    
    //Array overflow
    // let index = [12, 13, 14, 15];
    // let month = months[index[1]];

    // println!("{}", months);
}
