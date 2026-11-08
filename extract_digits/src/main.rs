fn main() {

    let s01: String = String::from("paramesh123skjd232");
    let chars: Vec<char> = s01.chars().collect();
    let mut s02: String = String::from("");

    for ch in chars{
        if ch.is_numeric() {
            s02.push(ch);
        }
    }
    println!("The numeric values are {}", s02);

    
}
