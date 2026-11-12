fn main() {

    let values: Vec<i32> = vec![10,2,30,4,5,6,7];

    // closure called track_min_max
    let track_min_max = |(min,max), &val| {

       let s; 
       let t;

        if val < min{
            s = val;
        }
        else {s = min}
        if val > min{
            t = val;
        }
        else {t = max}
        (s,t)
    };

    let (min, max) = values.iter().fold((1000000,0), track_min_max);

    println!("min: {}, max: {}", min, max);

}
