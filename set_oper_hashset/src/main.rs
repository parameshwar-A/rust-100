use std::collections::HashSet;
fn main() {

    let mut seta: HashSet<char> = HashSet::new();
    let mut setb: HashSet<char> = HashSet::new();

    seta.insert('a');
    seta.insert('b');
    seta.insert('c');

    setb.insert('a');
    setb.insert('c');
    setb.insert('d');


    let un = seta.union(&setb);
    println!("Union: {:#?}", un);

    let inter = seta.intersection(&setb);
    println!("Intersection: {:#?}", inter);
 
    let diff= seta.difference(&setb);
    println!("Difference: {:#?}", diff);



}
