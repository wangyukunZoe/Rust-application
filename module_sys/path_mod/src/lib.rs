mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
        //pub -> public 访问修饰符
    }
}

// Rust 默认的访问修饰权限是“私有的“
//声明一个函数：在饭店吃饭
// pub -> public 访问权限：公共的
pub fn eat_at_restaurant() {
    //使用绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    //使用相对路径
    front_of_house::hosting::add_to_waitlist();
}