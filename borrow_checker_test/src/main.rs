fn main() {

    let mut a: String = String::from("This String");
    {
    // Two mutable references cannot be created at same place
    // They can be in different scope sequentially 
    let c: &mut String = &mut a;
    println!("{}", c);
    // c goes out of scope after this!
    // So rustc allows to create another mutable reference after this scope
    }
    let d: &mut String = &mut a;
    println!("{}", d);

    let mut s: String = String::from("New String");
    {
    // Both immutable and mutable borrow cannot happen at same time
    // Multiple immutable creation can happen though since they are read-only steps
    let p: &String = &s;
    println!("{}",  p);

    // After this scope the read only gets cleared up so mutable reference can be created!
    }
    let q: &mut String = &mut s;
    println!("{}",  q);

}
