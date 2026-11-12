fn main() {

    let values: Vec<i32> = vec![89,103,99, 200, 110];

    // let capped_values: Vec<i32> = values.into_iter().filter(|v| *v>100).map(|va| 100).collect();
    let capped_values: Vec<i32> = values.into_iter().map(|va| if va > 100 {100} else {va}).collect();
    println!("{:#?}", capped_values);

}
