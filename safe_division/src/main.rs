fn main() {

    fn safe_division(num: f32, den:f32) -> Option<f32> {

        if den == 0.0{
            return None
        }

        let quo: f32 = num / den;
        Some(quo)
    }

    let result: Option<f32> = safe_division(10.0,0.0);
    match result {
        Some(val) => println!("The result is {}", val),
        _ => println!("The denominator is not valid")
    }
}
