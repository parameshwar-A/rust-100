fn main() {

    let s01: String = String::from("Parameshwar");

    let chars: Vec<char> = s01.chars().collect();
    let mut vowel: u32 = 0;
    let mut consonants: u32 = 0;
    for item in chars{
        if item == 'A' || item == 'E' || item == 'I' || item == 'O' || item == 'U' || item == 'a' || item == 'e' || item == 'i' || item == 'o' || item == 'u'{
            vowel += 1;
        }
        else{
            consonants += 1;
        }
    }
    println!("{}", s01);
    println!("Vowel count: {} \nConsonants: {}", vowel, consonants);
}
