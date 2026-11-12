fn main() {

    let x: Vec<i32> = vec![1,2,3,4,5];
    let y: Vec<i32> = vec![1,4,9,16,25];

    let coordinates: Vec<(i32,i32)> = x.into_iter().zip(y).collect();

    println!("{:#?}", coordinates);
}
