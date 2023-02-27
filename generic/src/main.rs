pub trait ISummary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub title: String,
    pub author: String,
    pub content: String,
    pub create: u32,
}

pub struct Tweet {
    pub head: String,
    pub content: String,
    pub favorite: u32,
    pub retweet: u32,
}

impl ISummary for Article {
    fn summarize(&self) -> String {
        return String::from(format!("{} is write by {}, create at {}, He(She) said: {}", self.title, self.author, self.create, self.content));
    }
}

impl ISummary for Tweet {
    fn summarize(&self) -> String {
        return String::from(format!("{} said: {}, this tweet gets {} favorites, and retweet for {} times", self.head, self.content, self.favorite, self.retweet));
    }
}

fn main() {
    let article = Article {
        title: String::from("I love apple"),
        author: String::from("Steve"),
        content: String::from(" is an American multinational technology company headquartered i"),
        create: 19900708,
    };

    let tweet = Tweet {
        head: String::from("I am king"),
        content: String::from("Lebron is the young king"),
        favorite: 1200,
        retweet: 10,
    };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize())
}
