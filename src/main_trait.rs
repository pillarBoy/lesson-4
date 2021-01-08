pub trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    author: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", &self.author, &self.text)
    }
}

fn main() {
    let tweet = Tweet {
        author: "pillar".to_string(),
        text: "morning".to_string()
    };

    println!("{}", tweet.summarize());
}
