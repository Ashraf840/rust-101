// Tut-17.2.0: Traits (Default Implementation/impl)

// Traits in Rust - by Let's Get Rusty; Video Ref: https://www.youtube.com/watch?v=T0Xfltu4h3A
// His tutorial is streched to "main_17_2_2.rs" file.

/*
 * Objectives
 * - Create three structs in external file (main_17_2_1.rs).
 * - Create a common trait for those structs (Summary) to summrize text contents.
 * - Import those structs into this file (main_17_2_0.rs) & define variables using those structs.
 * - Invoke the previously defined trait afterward creating those variables.
 */

 mod main_17_2_1;   // Since rust defines this file as module, thus this file requires to be explicitly defined as "mod mod main_17_2_1". After then, we can use all the things of that page.
 use main_17_2_1::{Tweet, FbPost, NewsArticle, Summary};    // ***Required to import structs & traits

fn main() {
    let tweet = Tweet {
        username: String::from("Yeasmin Akhter"),
        content: String::from("Oxygen levels are dropping in rivers across the US and central Europe"),
        reply: false,
        retweet: true,
    };

    let fb_post = FbPost {
        username: String::from("Shihab Uddin"),
        post: String::from("Get busy living or get busy dying"),
        likes: 264,
        poeple_reached: 2487,
    };

    let news_article = NewsArticle {
        author: String::from("Stephen King"),
        headline: String::from("Mass fish deaths will occur more often in rivers as UK gets hotter"),
        content: String::from("Large numbers of dead fish have been seen in many UK rivers in recent weeks, and researchers warn that climate change will lead to more such events."),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Fb post summary: {}", fb_post.summarize());
    println!("News article summary: {}", news_article.summarize());
}
