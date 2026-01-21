#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 为struct 实现方法
//用implement
impl Rectangle {

    //目前我们的借用的 self 没有使用所有权
    // 实际上你也可以使用所用权 比如说一个可变的所有权 &mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn area_mut(&mut self) -> u32 {
        self.width * self.height
    }

    //判断一个长方形能否容纳另一个长方形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //关联函数
    //类似于 JS 中的构造器或静态函数，或 Java 中“友元的概念friend
    //创建一个正方形，一种特殊的矩形
    fn sequare(size: u32) -> Rectangle {
        Rectangle { 
            width: size, 
            height: size, 
        }
    }
}


// impl Rectangle {
//      //判断一个长方形能否容纳另一个长方形
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }

//     //关联函数
//     //类似于 JS 中的构造器或静态函数，或 Java 中“友元的概念friend
//     //创建一个正方形，一种特殊的矩形
//     fn sequare(size: u32) -> Rectangle {
//         Rectangle { 
//             width: size, 
//             height: size, 
//         }
//     }
// }

fn main() {
    let rect = Rectangle {
        width: 30,
        height:50,
    };

    println!("{}", rect.area());

    println!("{:#?}", rect);

    let mut rect_mut = Rectangle {
        width: 60,
        height:50,
    };

    println!("{}", rect_mut.area_mut());

    println!("==============================");

    let rect1 = Rectangle {
        width: 30,
        height:50,
    };

    let rect2 = Rectangle {
        width: 10,
        height:40,
    };

    let rect3 = Rectangle {
        width: 35,
        height: 55,
    };
    
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
    // println!("{}", rect1.can_hold(&rect2));

    println!("{:#?}", rect);

    // 创建一个正方形
    let s = Rectangle::sequare(20);
    println!("Rectangle square area = {}", s.area() );
}
