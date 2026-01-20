struct User{

    username: String,
    email:String,
    sign_in_count:u64,
    active:bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 0,
    }

    //可用以下方式简化关键字的初始化
    // email,
    //username,
}

fn main() {

    println!("Hello world!");

    //将结构实例化
    //⚠️注意：Rust 要求初始化器里面的所有关键字都被实例化，缺一不可
    let user1 = User {
        email: String::from("wangyukun20011027@icloud.com"),
        username:String::from("Nikky"),
        active:true,
        sign_in_count:556,
    };

    let user_email = "wangyukun@163.com";
    let user_name = "wongZoe";

    build_user(user_email.to_string(), user_name.to_string());
}
