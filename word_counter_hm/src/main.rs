use std::collections::HashMap;
fn main() {

    let mut map = HashMap::new();


    let s01: String = String::from("The box is there The box is great");

    for word in s01.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map{
        println!("Word: {} Count: {}", key, value);
    }

}
