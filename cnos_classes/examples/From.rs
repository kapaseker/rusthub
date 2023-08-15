use std::str::FromStr;

#[derive(Debug)]
struct Person {
    age: u32,
}

impl Default for Person {
    fn default() -> Self {
        return Person { age: 0 };
    }
}

struct Student {
    age: u32,
}

impl From<Student> for Person {
    fn from(value: Student) -> Self {
        Person { age: value.age }
    }
}

impl FromStr for Person {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return if s.ends_with('s') {
            let parse_result: Result<u32, _> = s[0..(s.len() - 1)].parse();
            if let Ok(age) = parse_result {
                return Ok(Person { age });
            }
            return Err("Parse Error".to_string());
        } else {
            Err("Parse Error".to_string())
        };
    }
}

fn main() {
    let jason = Student { age: 21 };
    let people: Person = jason.into();

    println!("{:?}", people);

    println!("{:?}", "23s".parse::<Person>().unwrap_or_default());
    println!("{:?}", "x23s".parse::<Person>().unwrap_or_default());
}