// Traits (interfaces, prescriptions)
// are codeblocks, that extend and unite the functionality of struct, enum, generic...
// There is only one limitation about traits and that is that we can't implement external traits
// on external types. That limitation is known as coherence. 
// This rule ensures that other people’s code can’t break your code and vice versa. 

mod news_article;
mod summary;
mod vec2;

use news_article::*;
use summary::Summary;


// We can also use traits as parameters
// so that the function only takes a value, that is compliant with the trait as parameter.
// meaning: Your dumbass can't but anything else, than a type that implements Summary trait.
fn summarize_item(item: &impl summary::Summary) {
    println!(
        "Breaking news! {}",
        item.summarize()
    );
}

// The previous function works fine, but in nature it's just a syntax sugar
// in reality it looks like this function
fn sum_item<T: Summary>(item: &T) -> impl Summary {
    println!(
        "Breaking news! {}",
        item.summarize()
    );

    // And just as we can use it as a parameter
    // we can use it as a return type
    // so that we can only return types, that implements Summary
    // !!! However we can't return multiple types that implement Summary
    // it can only be one either Tweet or NewsArticle not both. 
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Typescript is dead"),
        location: String::from("new york"),
        author: String::from("turbo8"),
        content: String::from("hahahhaha"),
    };

    // The default way of calling these would be: 
    // article.summarize()
    // or 
    // tweet.summarize()
    // but because I am having hard time with modules, I am calling it this way... :(

    println!("1 new tweet: {}", summary::Summary::summarize(&tweet));
    println!("{}", summary::Summary::summarize(&article));

    // Example of the default implementation
    println!("{}", summary::Summary::summarize_default(&article));
    
}