// Create 3 structs (Tweet, FbPost, NewsArticle)

// Traits in Rust - by Let's Get Rusty; Video Ref: https://www.youtube.com/watch?v=T0Xfltu4h3A
// His tutorial is streched to "main_17_2_2.rs" file.

// This file's structs & traits are imported inside "main_17_2_0.rs" file.

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implement the trait (Summary) into "Tweet" data type
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: content: {}", self.username, self.content)
    }
}

pub struct FbPost {
    pub username: String,
    pub post: String,
    pub likes: u64,
    pub poeple_reached: u64,
}

// Implement the trait (Summary) into "FbPost" data type; but doesn't define any function, thus default implementation of trait will be applied here.
impl Summary for FbPost {}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

// Implement the trait (Summary) into "NewsArticle" data type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

// Build a trait whose generic behavior is shared among the 3 aforementioned data types.
// Also define a default body of the trait.
// Require to make it public since while the other datatypes (structs) are imported,
// they're automatically importing their implementation leaving the trait behind which is 
// also implemented on those structs, thus they need to be explicitly defined as imported 
// in that file (main_17_2_0.rs) & for that reason, this trait is required to be defined as 
// a publiic trait.
pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more...)")   // Default body
    }
}