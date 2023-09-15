// Tut-17.1: Traits

// Traits in Rust - by Let's Get Rusty; Video Ref: https://www.youtube.com/watch?v=T0Xfltu4h3A
// His tutorial is streched to "main_17_2_2.rs" file.

/*
 * Objectives
 * - Create a simple trait for two different structs (complex data structure)
 * - Implement that trait to both of the structs.
 * - Create two vars out of those structs.
 * - Invoke the two traits of two structs afterwards.
 */

/*
 * Q/A
 * - What is a trait?
 *   - Shared behaviors (methods) accross different types (structs).
 *   - Trait is more like "interface" in other languages, indeed w/ certain differences.
 */

 #![allow(dead_code)]


// Complex data structure-1
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// Implement the trait "Summary" to "Tweet" data type (struct)
impl Summary for Tweet {
    fn summerize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Complex data structure-2
struct NewsArticle {
    author: String,
    headline: String,
    content: String,
}

// Implement the trait "Summary" to "NewsArticle" data type (struct)
impl Summary for NewsArticle {
     fn summerize(&self) -> String {
         format!("{}, by {}", self.content, self.author)
     }
 }

// Define a trait of shared behavior ("summerize()")
trait Summary {
    // Only define a method signature; negate the method body
    fn summerize(&self) -> String;
}

fn main() {
    // Create Tweet type var
    let tweet = Tweet {
        username: String::from("Refoan Ahmed"),
        content: String::from(r##"Sharukh Khan saves bollywood by his movie "Jawan""##),
        reply: false,
        retweet: true,
    };
    println!("Tweet summary: {}", tweet.summerize());

    // Create NewsArticle type var
    let newsarticle = NewsArticle {
        author: String::from("Sitara Krishnakumar"),
        headline: String::from("Bollywood is again saved by Shahrukh Khan"),
        content: String::from("lorem10"),
    };
    println!("NewsArticle summary: {}", newsarticle.summerize());
}