fn main() {
    let num1 = "123";
    let num2 = 12;

    let num3 = num1.parse::<i32>().unwrap();
    let sum = num2 + num3;
    println!("{}", sum);
}
