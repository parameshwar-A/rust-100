fn main() {

    let reference = dangle();
    println!("{}", reference);
}

fn dangle() -> String {
    // Getting missing lifetime specifier if we use -> &String for returning
    // When the function completes this variable val gets out of scope. 
    // So what happens to the reference we pass. if we value get deleted, reference will be invalid
    // right ?
    // To avoid this dangling reference must not be used, instead return the actual value instead of
    // reference

    let val: String = String::from("is this Dangling reference string ?");

    val
}
