use std::collections::HashMap;
use std::fmt::Result;
use std::fmt;
use std::io;
use std::io::Result as IoResult;


// fn f1() -> fmt::Result {}
// fn f1() -> Result {}

// fn f2() -> io::Result {};
// fn f2() -> IoResult {};

fn main(){
    let mut map = HashMap::new();
    map.insert(1, 2);
}