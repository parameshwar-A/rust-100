fn main() {
    let s1: String = String::from("ele");
    let mut s2: String = s1.clone();
    reverse_string(&mut s2);
    if s1 == s2{
        println!("Palindrome: {}", s1);
    }
    else{
        println!("Not a Palindrome: {} {}", s1, s2);
    }
}

fn reverse_string(s01: &mut String){

   let mut chars: Vec<char> = s01.chars().collect();
   let mut s = 0;
   let mut e = chars.len() -1;

   if chars.len() == 0{
       return
   }
   while e>s{
       chars.swap(s,e);
       s+=1;
       e-=1;
   }
   s01.clear();
   s01.extend(chars);
}   
