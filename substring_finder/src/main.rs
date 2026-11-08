fn main() {

    let s01: String = String::from("welcome to the club");
    let s02: String = String::from("tot");

    let value: Option<usize> = find_sub(&s01, &s02);

    match value{
        Some(val) => println!("The index is {}", val),
        _ => println!("The substring not found")
    }

}
fn find_sub(s1: &str, sub: &str) -> Option<usize>{
        let slen: usize = sub.len();
        let sub_chars: Vec<char>= sub.chars().collect();
        let chs: char = sub_chars[0];
        for (ind, ch) in s1.chars().enumerate(){
            if ch == chs{
                let end: usize = ind + slen;
                if s1[ind..end] == sub[..]{
                    return Some(ind);
                }
            }
        }
        return None;
    }

