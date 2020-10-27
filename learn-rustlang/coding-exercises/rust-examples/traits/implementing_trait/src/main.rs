// trait which needs to be implemmented (like an interface in other languages)
pub trait Summary {
    fn summarize(&self) -> String;
}
// same as the above but with a default implemment if one is not supplied
pub trait DefaultSummary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
// for functions can be added in a trait and can be called from inside the trait
pub trait MoreSummary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// implemment the trait Summary
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let artcile = NewsArticle {
        headline: String::from("headlinehere"),
        location: String::from("locationhere "),
        author: String::from("namehere"),
        content: String::from("somecontent"),
    };
    println!("{:?}", artcile.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
