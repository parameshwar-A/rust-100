fn main() {

    struct Metric{
        value: i32,
        metric_name: String
    }

    struct Meter{
        value:i32,
    }

    trait Describe{
        fn get_value(&self) -> i32;
        fn get_metric_name(&self) -> String;
        fn describe(&self){
            println!("It is of value {} with metric as {}", self.get_value(), self.get_metric_name());
        }
    }

    impl Describe for Metric{

        fn get_value(&self) -> i32 {
            self.value
        }
        fn get_metric_name(&self) -> String{
            self.metric_name.clone()
        }
    }

    impl Describe for Meter{
        fn get_value(&self) -> i32 {
            self.value
        }

        fn get_metric_name(&self) -> String{
            "".to_string()
        }


        fn describe(&self){
            println!("The length is of {} meters", self.value);
        }

    }

    let m1: Metric = Metric{
        value: 23,
        metric_name: String::from("inches")
    };

    let met1: Meter = Meter{
        value: 34,
    };

    m1.describe();
    met1.describe();
}
