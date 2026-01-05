fn main() {
    //boolean
    let b1: bool = true;

    // unsigned integers
    let i1:u8 = 1;  //u8 unsigned
    let i2:u16 = 1;  
    let i3:u32 = 1;  
    let i4:u64 = 1;  
    let i5:u128 = 1;  

    //signed integers
    let i6:i8 = 1;  //u8 unsigned
    let i7:i16 = 1;  
    let i8:i32 = 1;  
    let i9:i64 = 1;  
    let i10:i128 = 1;  

    //floating point numbers
    let f1:f32 = 1.0;
    let f2:f64 = 1.0;

    //platform specific integers
    let p1: usize = 1;
    let p2: isize = 1;

    //characters &str, and String
    let c1: char = 'c';
    let s1: &str = "hello"; //&str 字符串切片
    let s2: String = String::from("hello");

    //复合数据类型 compound 
    //arrays
    let a1: [i32; 5] = [1, 2, 3, 4, 5];

    let i1: i32 = a1[4];

    //元组
    //tuples

    let t1: (i32, i32, i32) = (1, 2, 3);
    let t1: (i32, f64, &str) = (5, 5.5, "5");

    let s1: &str = t1.2;    //t1.2 -> print("2")打印元组t1中的第二个元素，并将其转换为字符串
    //元组的另一种写法
    let (i1, f1, s1): (i32, f64, &str) = t1;

    let unit: () = ();

    //type aliasing 类型别名
    type Age = u8;
    
    let a1: Age = 57;

    println!("Rust Data Structure");
}
