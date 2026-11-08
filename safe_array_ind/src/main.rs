fn main() {
    let values: [i32;5] = [0;5];
    let val: &i32 = &values[0];
    println!("{}", val);

    fn safe_get(ind: i32, value: &[i32;5]) -> Option<&i32>{
        if ind < 0 || ind >= value.len().try_into().unwrap() {
            return None
        }
        let vl: &i32= &value[ind as usize];
        Some(vl)
    }

    let res: Option<&i32> = safe_get(1, &values);
    match res {
        Some(val) => println!("The value in index is {}", val),
        _ => println!("Invalid Index")
    }
}
