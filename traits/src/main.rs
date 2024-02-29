pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{} by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!(
//             "username: {} and the content: {}",
//             self.username, self.content
//         )
//     }
// }

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
fn main() {
    let na = NewsArticle {
        headline: String::from("Breaking News!!!"),
        location: String::from("Breaking News!!!"),
        author: String::from("Breaking News!!!"),
        content: String::from("Breaking News!!!"),
    };
    // println!("{}", na.summarize());

    let tweet = Tweet {
        username: String::from("It'sMe"),
        content: String::from("It'sMe"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize_author());
}
