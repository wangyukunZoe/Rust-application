fn main() {
    //不可恢复的 panic！
    // panic!("CRASH AND BURN!");

    let v = vec![1,2,3];

    v[99];

    //macOS
    //env RUST_BACKTRACE=1 && cargo run  
    // env RUST_BACKTRACE=full && cargo run
    //RUST_BACKTRACE=1 cargo run
    //RUST_BACKTRACE=full cargo run


    //Windows
    //set RUST_BACKTRACE=1 && cargo run
    /*
        thread 'main' (92474) panicked at src/main.rs:7:6:
        index out of bounds: the len is 3 but the index is 99
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */
}
