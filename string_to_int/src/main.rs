use std::io;
fn main() {
    let mut inp: String = String::new();

    io::stdin().read_line(&mut inp).expect("Failed to read line");

    let inp: u32 = match inp.trim().parse(){
        Ok(val) => val,
        Err(E) => panic!("Error converting the value {}", E)
    };
}
