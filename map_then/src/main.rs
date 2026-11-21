fn main() {

    let v: Vec<i32> = vec![89,99, 75, 20, 100];

    let v2: Vec<i32> = v.iter().map(|x| x+10).collect();
    let value = v.get(0).and_then(value_to_string);
    for item in &v2{
        println!("{}", item);
    }
    println!("{:?}", value);
}

fn value_to_string(val: &i32) -> Option<String>{
    Some(val.to_string())
}
