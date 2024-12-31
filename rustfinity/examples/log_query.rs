pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        // 🎁 Your code here! 🎁
        let logs = self.search(keyword).iter().map(|arg: &&String| String::as_str(*arg)).collect::<Vec<_>>().join("\n");
        std::fs::write(path, logs)
    }
}
fn main() {
    let a = &vec!["hello".to_string(), "world".to_string()];
    let query = LogQuery::new(a);
    let search = query.search("hello");
}