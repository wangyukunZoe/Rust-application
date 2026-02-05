// 结构体泛型
struct Point<T, U> {
    x: T,
    y: U,
}

// 枚举泛型
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct PointMethod<T> {
    x: T,
    y: T,
}

impl<T> PointMethod<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

impl PointMethod<i32> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}

fn main() {
    let integer = Point { x: 5, y: 1.0 };
    let float = Point { x: 1.0, y: 4.0 };
}
