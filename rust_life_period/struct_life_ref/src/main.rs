struct ImportantExcerpt<'a> {
    part: &'a str,
} //part类型是引用类型，字符串切片  part的生命周期必须比ImportanrExcerpt要长，要不就会是空🈳结构体
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("Could not found a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
