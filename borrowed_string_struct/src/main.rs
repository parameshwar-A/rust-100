fn main() {

    struct Token<'a>{
        position: i32,
        text: &'a str
    }

    let full_string: String = String::from("Welcome to the next program");

    let tk1 = Token{
        position: 0,
        text: &full_string[0..3]
    };

    println!("{} is the text in the position of {}", tk1.text, tk1.position);
}
