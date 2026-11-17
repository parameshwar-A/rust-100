fn main() {

    let full_string: String = String::from("Welcome to the club");

    let result = longest(&full_string[2..4], &full_string[3..9]);
    println!("{} is the longest string slice", result);


}

fn longest<'a>(x: &'a str, y: &'a str)  -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    else{
        return y;
    }
}
