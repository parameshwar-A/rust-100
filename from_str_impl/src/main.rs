use std::str::FromStr;

fn main() {
    struct Profile{
        first_name: String,
        last_name: String,
        dob: String
    }

    #[derive(Clone)]
    struct ProfileParsingError;

    impl FromStr for Profile{

        type Err = ProfileParsingError;
        fn from_str(s: &str) -> Result<Self, Self::Err>{
            let mut parts= s.split(":");
            let first = parts.next().ok_or(ProfileParsingError)?;
            let last = parts.next().ok_or(ProfileParsingError)?;
            let date = parts.next().ok_or(ProfileParsingError)?;
            Ok(Profile{first_name: first.to_string(), last_name: last.to_string(), dob: date.to_string()})
        }
    }

    match "karthik:nagarajan:23-10-1990".parse::<Profile>(){
        Ok(name) => println!("{} {} {}", name.first_name, name.last_name, name.dob),
        Err(_) => println!("invalid profile format")
    };



}
