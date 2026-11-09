use std::collections::HashMap;
fn main() {
    let mut map: HashMap<i32, i32> = HashMap::new();

    let myvec: Vec<i32> = vec![10,20,30,45,50];

    let target:i32 = 65;
    let mut comp: i32;
    let mut ind: i32 = 0;

    for elem in &myvec{

        comp = target-elem;
        
        match map.get(elem){
            Some(val) => {
                println!("{} {}",val, ind);
                break;
            }
            None => {
                map.insert(comp, ind);
            }
        }
        ind+=1;
    }

    
}
