use std::thread;
use std::sync::mpsc;


fn main() {

    let a: i32 = 10;
    let b: i32 = 11;
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {

        let result = odd_or_even(a);
        tx.send((result.clone(), a.to_string())).unwrap();
    });
    thread::spawn(move || {
        let result = odd_or_even(b);
        tx1.send((result.clone(), b.to_string())).unwrap();
    });

    for (val, num)in rx{
    println!("RX: {} is {} ", num, val);
    }
}


fn odd_or_even(val: i32) -> String{
    if val %2 ==0 {
        return "even".to_string();
    }
    return "odd".to_string();
}
