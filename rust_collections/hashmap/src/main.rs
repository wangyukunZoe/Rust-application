use std::collections::HashMap;

fn main() {
    //适用于创建有键值对数值的情况
    //HashMap内不能为空🈳
    let mut scores = HashMap::new();
    //也可以显示表明数据类型，这里不这么做了，因为引入动态数据的时候会出现问题

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    //==========================================
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];

    let scores: HashMap<_,_> = 
        teams.iter().zip(intial_scores.iter()).collect();   //创建一个元组的数组

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // map.insert(field_name, field_value);    //所有权会丧失，以后将不能访问对应的键和值
    //  如果想继承值和键所有权，要传引用才行
    map.insert(&field_name, &field_value);

    // println!("{}, {}", field_name, field_value); //cannot do this, its owning have been borrowed
    println!("{}, {}", field_name, field_value);

}
