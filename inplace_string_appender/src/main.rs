fn main() {
    let mut s1: String = String::from("My name is ");

    println!("Before: {s1}");
    inplace_appender(&mut s1);
    println!("After: {s1}");


}

fn inplace_appender(s01: &mut String){
    s01.push_str("Parameshwar Arunachalam");
}
