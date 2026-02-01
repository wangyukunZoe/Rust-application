use std::error::Error;
use std::fs::File;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.md")?;
    Ok(())
}
