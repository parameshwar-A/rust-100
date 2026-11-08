fn main() {


    let s01: String = String::from("Welcome to the test string");

    let results: [u8;52] = char_counter(&s01);
    for (ind, value) in results.iter().enumerate(){
        println!("{} {}", (ind as u8 +65) as char, value);
    }
}

fn char_counter(s02:&str) -> [u8;52]{

        let mut counter:[u8;52] = [0;52]; 

        for ch in s02.chars(){
            let mut val = ch as u8;
            val = val %65;
            let ind = val as usize;
            counter[ind] = counter[ind] + 1;
        }

        counter
    }

