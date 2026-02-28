fn main() {
    let x = 4;
    let equal_to_x = |z| z == x; //闭包可以捕获外部变量 

    /*
    fn equal_to_x(z: i32) -> bool {
        //普通函数不能捕获外部的变量，闭包可以
        z == x
    }
     */
    // 消耗指的是在捕获变量的同时也获取其所有权，闭包结束后变量也就移出了作用域一起被销毁
    // FnOnce{ FnMut { Fn } }
    // 这个地方老师应该讲错了 所有实现了fn的都实现了fnmut，那fnmut就应该包含fn了
    let y = 4;

    assert!(equal_to_x(y))
}
