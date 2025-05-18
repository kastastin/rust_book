use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: "bob".to_string(),
        content: "content example".to_string(),
        reply: false,
        retweet: false,
    };

    println!("tweet.summarize = {}", tweet.summarize());

    let article = NewsArticle {
        headline: "Penguins win the Stanley Cup Championship!".to_string(),
        location: "Pittsburgh, PA, USA".to_string(),
        author: "Iceburgh".to_string(),
        content: "The Pittsburgh Penguins are the best hockey team in the NHL.".to_string(),
    };

    println!("artice.summarize = {}", article.summarize());
}
