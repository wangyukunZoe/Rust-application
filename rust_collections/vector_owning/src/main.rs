#[warn(unused)]
#[warn(unused_mut)]

fn main() {
    // !‼️不能在同一作用域内同时拥有可变和不可变引用
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];  //由于这里引用了不可变的借用导致错误
    /*
        这是因为 vector 内部在分配内存时是离散的，当结构需要更多的内存时，vector 会通过 reallocate 申请
        更多的内存，并获得不同的地址，此时如果把它赋予一个不可变的变量，vector 就不能再往内存里动态申请地址了
        因此再往 vector 里面加一定是不行的
        这就相当于：你的家里多了一口人，现在的房子🏠住不下了，也无法扩建，需要换其他的房子住，其他房子的地址一定与
        源地址不一样，如果还是固定不变的地址一定是错误的
     */
    // v.push(6);

    // 通过循环获得 vector 的遍历
    for i in &v {
        println!("{}", i);
    }
    println!("The first element is {}", first);
}
