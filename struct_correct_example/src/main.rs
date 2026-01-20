//  This code will get an Error!
// Correct Example
//
struct User1 {
    //这里一定会触发一个错误，因为 Rust 要求引用必须在生命周期内
    username: &str,
    email: &str,
    sign_in_count:u64,
    active:bool,
}

fn main() {
    let user2 = User {
        email: "fdsa",
        username:"fds",
        active:true,
        sign_in_count:556,
    };
}
