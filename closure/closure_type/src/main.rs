fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);  //闭包模板只能使用一次
}
