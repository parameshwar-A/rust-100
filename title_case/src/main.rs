fn main() {
    let s01: String = String::from("Welcome to the club");

    let words: Vec<&str> = s01.split_whitespace().collect();
    let mut s02: String = String::from("");

    for word in words{
        for (i,item) in word.chars().enumerate(){

            if i ==0 {
                s02.push(item.to_ascii_uppercase());
            }
            else{
                s02.push(item);
            }
        }
        s02.push(' ');

    }

    println!("The normal string is {}", s01);
    println!("The capitalized string is: {}", s02);

}
