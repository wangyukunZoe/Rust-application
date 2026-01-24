fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        // struct 结构里面的字段默认是私有的
        pub toast: String,
        pub seasonal_fruit: String,
    }

    // 关联函数
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"), 
            }
        }
    }
}

mod back_of_house2 {
    
    // 枚举已变为公共项
    pub enum Appetizer {
        Soup,
        Salad,
        //公共枚举里面的所以变体都是公共的
        //
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    meal.seasonal_fruit = String::from("blueberries");
}