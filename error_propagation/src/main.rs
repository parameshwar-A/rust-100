use std::io::{self, Read};
use std::fs::File;

fn main(){
    match read_username_from_file() {
        Ok(result) => println!("{}", result),
        Err(error) => println!("Error")
    };
}

fn read_username_from_file() -> Result<String, io::Error>{

    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

}

