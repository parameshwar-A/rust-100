fn main() {

    let s1: String = String::from("Welcome home!");
    string_length_inspector(&s1);
    println!("The value of the string is: {s1}");

}

fn string_length_inspector(s01: &String) {

    let len: usize = s01.len();
    println!("The length of string is: {len}");

}
