use std::{env, process};
use std::error::Error;
use std::fs;

pub fn search<'a>(ignoreCase: bool, query: &str, contents: &'a str) -> Vec<&'a str> {
    return if ignoreCase {
        contents.lines().filter(|x| x.to_lowercase().contains(&query.to_lowercase())).collect()
    } else {
        contents.lines().filter(|x| x.contains(query)).collect()
    };
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    for x in search(config.ignore, &config.query, &contents) {
        println!("{x}")
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    query: String,
    path: String,
    ignore: bool,
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        return self.ignore == other.ignore && self.query == other.query && self.path == other.path;
    }
}

impl Config {
    pub fn build(mut argIter: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        argIter.next();

        let mut query: Option<String> = None;
        let mut path: Option<String> = None;
        let mut ignore: bool = false;

        for x in argIter {
            if x.starts_with('-') {
                if x == "-i" {
                    ignore = true
                }
            } else {
                if query == None {
                    query = Some(x)
                } else if path == None {
                    path = Some(x)
                }
            }
        }

        return Ok(Config {
            query: query.unwrap(),
            path: path.unwrap(),
            ignore,
        });
    }

    // pub fn build(str: &[String]) -> Result<Config, &'static str> {
    //     if str.len() < 3 {
    //         return Err("arguments size is too short");
    //     }
    //
    //     let mut query: Option<String> = None;
    //     let mut path: Option<String> = None;
    //     let mut ignore: bool = false;
    //
    //     let mut index = 0;
    //
    //     for x in str {
    //         if index > 0 {
    //             if x.starts_with('-') {
    //                 if x == "-i" {
    //                     ignore = true
    //                 }
    //             } else {
    //                 if query == None {
    //                     query = Some(x.clone())
    //                 } else if path == None {
    //                     path = Some(x.clone())
    //                 }
    //             }
    //         }
    //
    //         index += 1;
    //     }
    //
    //     return Ok(Config {
    //         query: query.unwrap(),
    //         path: path.unwrap(),
    //         ignore,
    //     });
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testSearch() {
        let query = "saw";
        let contents = "\
        I saw the fake one first, \n
        years ago, printed out in a report tacked on to a court filing out of New York City. \n
        There were two pictures on the first page, \n
        two sides of a painting, back and front. \n
        On the left, two rectangles, \n
        black over crimson on a background \n
        of lighter red. On the right, \n
        a wooden stretcher bisected by a crossbar. \n
        The edges of a canvas, folded over and stapled, \n
        were visible along the edge. \n
        There was a name on the back, too, \n
        and a date, written in fuzzy, \n
        impasto caps";

        assert_eq!(vec!["I saw the fake one first, "], search(false, query, contents));
        assert_eq!(vec!["I saw the fake one first, "], search(true, "SAW", contents));

        let testArg = vec![String::from("ab"), String::from("nothing"), String::from("good"), String::from("cd")];
        assert_eq!(Config::build(&testArg).unwrap(), Config { query: String::from("nothing"), path: String::from("good"), ignore: false });

        let testArg = vec![String::from("ab"), String::from("nothing"), String::from("good"), String::from("-i")];
        assert_eq!(Config::build(&testArg).unwrap(), Config { query: String::from("nothing"), path: String::from("good"), ignore: true })

        // assert_eq!(Config { query: String::from("nothing"), path: String::from("good"), ignore: false }, Config::build(&testArg).unwrap());
    }
}