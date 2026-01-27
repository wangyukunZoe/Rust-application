fn main() {
    // 不带任何初始值的 vector
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    // 与任何其他struct一样，当vector离开作用域后
    // 它就被清理掉了
    // 它的所有元素也被清理掉了
    // 这在读取关于 verctor 的引用或索引的方式会变得复杂

    //


}
