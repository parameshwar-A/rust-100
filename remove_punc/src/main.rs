fn main() {

    let s01: String = String::from("hello, there! I am. Thrilled..");

    let s02:String = s01.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect();

    println!("The processed string is: {}", s02);


}
