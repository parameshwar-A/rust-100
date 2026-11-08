fn main() {

    let s01: String = String::from("The quick brown fox jumps over the lazy dog");

    let chars: Vec<&str> = s01.split_whitespace().collect();

    let mut count: u16 = 0;

    for _word in chars{
        count+=1;
    }

    println!("word count is {}", count);

}
