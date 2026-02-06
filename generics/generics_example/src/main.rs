// fn largest<T: std::cmp::PartialOrd + Clone>(list: &[T]) -> T {
//     let mut largest = list[0].clone();
//     for item in list.iter() {
//         if item > &largest {
//             // > std::cmp::PartialOrd
//             largest = item.clone();
//         }
//     }
//     largest
// }

// 上面是比较麻烦的写法，我们换一种比较简单的写法
// 这种情况下，直接引用&即可
fn largest<T: std::cmp::PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > &largest {
            // > std::cmp::PartialOrd
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let char_list = vec!['y', 'm', 'a', 'q', 'p'];
    let result = largest(&char_list);
    println!("The largest number is {}", result);

    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest(&str_list);
    //因为 String 数据目前存储在 stack 上，没有存储在堆
    //heap 上，现在需要让他存储在 heap 上
    println!("The largest word is {}", result);
}

/*


/// # Panics
///
/// In this implementation, the `to_string` method panics
/// if the `Display` implementation returns an error.
/// This indicates an incorrect `Display` implementation
/// since `fmt::Write for String` never returns an error itself.
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: fmt::Display + ?Sized> ToString for T {
    #[inline]
    fn to_string(&self) -> String {
        <Self as SpecToString>::spec_to_string(self)
    }
}

#[cfg(not(no_global_oom_handling))]
trait SpecToString {
    fn spec_to_string(&self) -> String;
}

#[cfg(not(no_global_oom_handling))]
impl<T: fmt::Display + ?Sized> SpecToString for T {
    // A common guideline is to not inline generic functions. However,
    // removing `#[inline]` from this method causes non-negligible regressions.
    // See <https://github.com/rust-lang/rust/pull/74852>, the last attempt
    // to try to remove it.
    #[inline]
    default fn spec_to_string(&self) -> String {
        let mut buf = String::new();
        let mut formatter =
            core::fmt::Formatter::new(&mut buf, core::fmt::FormattingOptions::new());
        // Bypass format_args!() to avoid write_str with zero-length strs
        fmt::Display::fmt(self, &mut formatter)
            .expect("a Display implementation returned an error unexpectedly");
        buf
    }
}

*/

/*
if *item > largest {
    largest = *item;
}
*/
// 消除重复代码
