fn main(){

    let s1: String = String::from(" welcome home ");
    let result: &str = trim_whitespace(&s1);
    println!("Result of trim is \n{}", result)

}

fn trim_whitespace(s01: &str) -> &str {

    let bytes = s01.as_bytes();

    let mut start = 0;

    while start < bytes.len() && bytes[start] == b' ' {
        start += 1;
    }

    if start == bytes.len(){
        return ""
    }

    let mut end = bytes.len();

    while end > start && bytes[end-1] == b' '{
        end -= 1;
    }
    &s01[start..end]
}
