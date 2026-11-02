fn main() {
    let mut s: String = String::from("Welcome");
    reverse_string(&mut s);
    println!("{}", s);
}

fn reverse_string(s01: &mut String) {


    let mut chars: Vec<char> = s01.chars().collect();
    let mut s: usize = 0;
    let mut e: usize = chars.len()-1;

    if chars.is_empty(){
        return
    }

    let mut temp;
    while e>s {
        // Alternative in built usage instaed of temp: chars.swap(e,s);
        temp = chars[s];
         chars[s] = chars[e];
         chars[e] = temp;
        e-=1;
        s+=1;
    }

    s01.clear();
    s01.extend(chars);
}
