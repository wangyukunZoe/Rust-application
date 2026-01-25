mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn some_function(){}

    }
}

use crate::front_of_house::hosting;

//可以直接引用但不推荐 - 代码太多了
use crate::front_of_house::hosting::add_to_waitlist;


// use front_of_house::hosting;
//可使用相对路径
//  下面这个相当于上面使用use的方式
//另外 use 关键字的使用默认也是 private
// mod hosting {}

fn main() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // hosting::some_function();
}
