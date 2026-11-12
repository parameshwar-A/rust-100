fn main() {

    let mut myarray: Vec<i32> = vec![10,34,29,36,94];


    // Immutable borrow
    let printer = || {

        for val in &myarray{
            print!("{} ,", val);
        }
        println!("");
    };

    // Calling the immutable borrow
    printer();

    // still accessible after the immutable borrow
    println!("?{:#?}", myarray);



    // Mutable borrow
    let mut square = || {

        for val in &mut myarray{
            *val = *val * *val;
        }
    };

    square();
    // still accessible after the mutable borrow
    println!("?{:#?}", myarray);


    // Moved 
    //
    let _vector2 = move || {

        for val in &myarray{
            println!("{} ", val);
        }

    // myarray lives here but not outside
    };


    // won't be accessible after the move
    // below line with throw error
    // println!("?{:#?}", myarray);


    
    
}
