fn main() {

    // let a: String = String::from("1234567891");
    // let a: String = String::from("123");
    let a: String = String::from("123456789j012");


    match validate(&a){
        Ok(_) => println!("Great work!"),
        Err(MyError::MinimumLengthError) =>  println!("Correctly Identified length error"),
        Err(MyError::InvalidInputError) => println!("Correctly identified invalid error")
    }

}

#[derive(Debug)]
enum MyError{
    MinimumLengthError,
    InvalidInputError
}



fn validate(a: &str) -> Result<bool, MyError>{
    if a.len() < 8{
        return Err(MyError::MinimumLengthError);
    }

    if a.len() == 10{
        return Err(MyError::InvalidInputError);
    }
    Ok(true)
}
