fn main() {

    struct Counter{
        value: i32,
        limit: i32
    }

    impl Iterator for Counter{

            type Item = i32;

            fn next(&mut self) -> Option<Self::Item>{

                if self.value <= self.limit{
                    let val = self.value;
                    self.value +=1;
                    return Some(val);
                }
                else{
                    return None;
                }
            }

    }


    let c: Counter = Counter{value:0,limit:5};

    for b in c{
        println!("{}", b);
    }

}
