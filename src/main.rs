//with ? operator
use std::fs::File;
use std::io::{self, Read};

fn read_the_file(file_name: &str) -> Result<String, io::Error>{
    let mut username = String::new();

    File::open(file_name)?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    let _read_file = read_the_file("hello.txt");
    println!("{:?}",_read_file);
}
