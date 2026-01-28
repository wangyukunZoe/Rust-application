enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

//类似于 C 语言中的UNION 和 enum,只不过 Rust 这样做会更安全🔐

fn main() {
    // 这里 vector 需要提起指导需要存储哪种数据类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
