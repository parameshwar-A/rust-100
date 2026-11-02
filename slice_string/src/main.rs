fn main() {

    let s1: String = String::from("Hello World!");

    let result: &str = first_word_fetcher(&s1);

    println!("The first word from split is: {result}");

}

fn first_word_fetcher(s01: &String) -> &str {

    let bytes= s01.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s01[0..i];
        }
    }
    &s01[..]
}
