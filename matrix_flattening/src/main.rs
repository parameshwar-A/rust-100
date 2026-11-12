fn main() {
    let matrix: Vec<Vec<i32>> = vec![vec![1,2,3],vec![4,5,6]];

    let flatt: Vec<i32> = matrix.into_iter().flat_map(|x| x).collect();

    println!("{:#?}", flatt);
}
