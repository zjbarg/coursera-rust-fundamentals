use regex::RegexBuilder;
use std::env;

#[derive(Debug)]
struct FileSize(u64);

impl FileSize {
    fn parse(input: &String) -> Result<Self, String> {
        let re = RegexBuilder::new(r"^\s*(\d+)\s?(b|kb|mb|gb)?\s*$")
            .case_insensitive(true)
            .build()
            .unwrap();

        let captures = re.captures(&input);

        if let None = captures {
            return Err("Bad input".to_string());
        }

        let captures = captures.unwrap();

        let val = match captures.get(1).unwrap().as_str().parse::<u64>() {
            Ok(number) => number,
            Err(_) => return Err(String::from("Too Big!")),
        };

        if let Err(_) = captures.get(1).unwrap().as_str().parse::<u64>() {
            return Err(String::from("Too Big!"));
        }

        let suffix = match captures.get(2) {
            Some(group) => group.as_str(),
            None => "b",
        };

        let file_size = match suffix.to_lowercase().as_str() {
            "b" => FileSize::from(val, 0),
            "kb" => FileSize::from(val, 3),
            "mb" => FileSize::from(val, 6),
            "gb" => FileSize::from(val, 9),
            _ => panic!("cannot possibly get here!"),
        };

        Ok(file_size)
    }
    fn from(value: u64, exp: u32) -> Self {
        FileSize(value * (10 as u64).pow(exp))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("missing input");
    }

    match FileSize::parse(&args[1]) {
        Ok(fs) => println!("{:?}", fs.0),
        Err(_) => println!("Bad input: {}", args[1]),
    }
}
