fn main() {
    let mut s1 = String::from("Hello");
    let len = calculate_length(&mut s1);

    println!("The length of {} is {}.", s1, len);

    let mut s2 = String::from("Hello");
    let smut1 = &mut s2;
   //error: //  let smut2 = &mut s2;

    // println!("The length of {} is {}.", smut1, smut2);
    //可变引用有一个重要的限制，在特定作用域内，对某一块数据，只能有一个可变引用
    //这样做的好处是可以防止数据竞争
    //以下三种行为会引发数据竞争
    /*
        两个或多个指针同时访问同一个数据
        至少有一个指针用于写入数据
        没有使用任何机制来同步对数据的访问
     */

    //可以通过创建新的作用域，来允许非同时的创建多个可变引用
    let mut s3 = String::from("Hello");
    {
        let smut3 = &mut s3;
    }
    
    let smut4 = &mut s3;

    //❕不可以同时拥有一个可变引用和一个不可变的引用
    //多个不可变的引用是可以的
    let mut s5 = String::from("Hello");
    let _r1 = &s5;
    let _r2 = &s5;
    //let smut6 = &mut s5;
    //cannot borrow `s5` as mutable because it is also borrowed as immutable
    //mutable borrow occurs here
   // println!("{} {} {}", _r1, _r2, smut6);

}

//传递了一个引用
//引用就是允许你引用某些值而不获得其所有权
//那么我们把引用作为函数参数这个行为称为借用
fn calculate_length(s: &mut String) -> usize {
    s.push_str(",World!");
    s.len()
}

// 在没有设置可变引用的条件下，这一段是不可用的
// s.push_str(",World!");
    //error: could not compile `borrow_reference` (bin "borrow_reference") due to 1 previous error
