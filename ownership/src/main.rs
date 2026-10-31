fn main() {

    // main scope
    let a: i32 = 5;

    {
        println!("Sub Scope:The value of a is {a}");
        // sub scope
        let b: i32 = 10;
    }
    println!("Main Scope: The value of a is {a}");
//    println!("The value of b is {b}")


    let c: i32 = a;
    println!("After C creation: The value of a is {a}");
    println!("The value of c is {c}");

    // String Type: Heap
    //
    let s1: String = String::from("hello");
    println!("Before s2: The value of s1: {s1}");
    let s2 = s1;
    // After changed ownership to s2, the s1 will go out of scope
    // Below line will throw error 
    // println!("After s2: The value of s1: {s1}");
    println!("The value of s2: {s2}");

    // Below function will take s2 into that scope
    just_print(s2);

    // After execution, s2 will be out of scope since it moved to just_print function
    // Below line will throw error
    // println!("After function call: The value of s2: {s2}");

    let s3: String = String::from("world");
    println!("Before func call: The value of s3: {s3}");

    // This borrow function will take only the read only copy of the value
    // This is done by & symbol. Note: Borrow is read-only
    borrow_just_print(&s3);

    println!("After func call: The value of s3: {s3}");


    // Below line throws error since the value is borrowed, but trying to edit it
    // Borrow is read only state
    borrow_edit_print(&s3);


}

fn just_print(s01: String){
    println!("The value of s01: {s01}");
}

fn borrow_just_print(s01: &String){
    println!("The value of s01: {s01}");
}

fn borrow_edit_print(s01: &String){
    s01.push_str("Welcome");
    println!("The value of s01: {s01}");
}
