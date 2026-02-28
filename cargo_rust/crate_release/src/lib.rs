//! # Closure Crate
//!
//! `closure` is a collection of utilities to make performing
//! calculations more convenient.

/// Add one to the number given
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crate_release::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}
// 利用 cargo doc来生成文档
/*
wangyukun@wangyukundeMacBook-Pro-3 crate_release % cargo doc                                                           [0]
 Documenting crate_release v0.1.0 (/Users/wangyukun/Desktop/develop/rust/RustLearningNotes/Rust-application/cargo_rust/crate_release)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
   Generated /Users/wangyukun/Desktop/develop/rust/RustLearningNotes/Rust-application/cargo_rust/crate_release/target/doc/crate_release/index.html
wangyukun@wangyukundeMacBook-Pro-3 crate_release % cargo doc --open                                                    [0]
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Opening /Users/wangyukun/Desktop/develop/rust/RustLearningNotes/Rust-application/cargo_rust/crate_release/target/doc/crate_release/index.html
wangyukun@wangyukundeMacBook-Pro-3 crate_release %
*/
