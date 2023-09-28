use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, Write},
};

#[allow(dead_code)]
// output something into file
// this function is used to debug.
pub fn log<T: Display>(content: T) -> io::Result<()> {
    File::create("log.txt")?;
    let mut file = OpenOptions::new().write(true).open("log.txt")?;
    file.write(format!("{}", content).as_bytes())?;
    file.flush()?;
    return Ok(());
}
