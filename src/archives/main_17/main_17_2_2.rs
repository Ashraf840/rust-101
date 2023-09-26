// Tut-17.2.2: Traits (Default implementations which invokes other methods inside the same trait definition)

// Traits in Rust - by Let's Get Rusty; Video Ref: https://www.youtube.com/watch?v=T0Xfltu4h3A
// His tutorial is streched to this file (main_17_2_2.rs).

/*
 * Objectives
 * - Create 2 structs.
 * - Define a trait for those structs.
 *      - Inside that trait, a default implmentation body will be defined along with calling another method inside that trait definition.
 * - Implement the trait with a custom overridden method in one struct leaving another one with just the defult implementation.
 */

 #![allow(dead_code)]

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// Define (implement) both the functions' body; similar to function overriding
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("Tweeted by @{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

struct NewsArticle {
    author: String,
    headline: String,
    content: String,
}

// Define (implement) the "summarize_author()" func body, since the trait only has it's func signature
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

trait Summary {
    fn summarize_author(&self) -> String;   // Function Signature
    
    // Default function body
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("Yeasmin Akhter"),
        content: String::from("Oxygen levels are dropping in rivers across the US and central Europe"),
        reply: false,
        retweet: true,
    };

    let news_article = NewsArticle {
        author: String::from("Stephen King"),
        headline: String::from("Mass fish deaths will occur more often in rivers as UK gets hotter"),
        content: String::from("Large numbers of dead fish have been seen in many UK rivers in recent weeks, and researchers warn that climate change will lead to more such events."),
    };

    // Invoke the trait defined implemented method of the structs
    println!("{}", tweet.summarize_author());
    println!("Tweet summary: {}", tweet.summarize());
    println!("News article summary: {}", news_article.summarize());
}