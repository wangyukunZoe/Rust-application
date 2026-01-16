fn main() {
    // loop{    
    //     //一直循环的 again
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop{
        counter += 1;
        // println!("Again!");

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF 🚀");

    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    println!("==========For Loop===========");

    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //321倒计时
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF🚀");

}
