use std::fmt::Display;

pub struct NewArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewArticle {

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

// pub fn notify(item: &impl Summary) -> String {
//     format!("Breaking news! {}", item.summarize())
// }

pub fn notify<T: Summary>(item: &T) -> String {
    format!("Breaking news! {}", item.summarize())
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    
    fn summarize_author(&self) -> String {
        format!("")
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

fn return_summarization() -> impl Summary {
    Tweet { 
        username: String::from("horse_wbooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    }
}

pub trait Summary {

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String { 
        format!("Read more ...")
    }
}

struct Pair<T> {
    x: T, 
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> /*Self*/ {
        Self {x, y}
    }
}

// impl<T: Display> ToString for T {
//     // --snip
// }

impl<T> Pair<T>
    where T: Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else { 
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@mohammdmahdi"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    };

    let artical = NewArticle {
        author: String::from("Mohammdmahdi"),
        headline: String::from("The Sky is Falling"),
        content: String::from("The sky is Not actully falling"),
    };

    println!("Article Summary: {}", notify(&artical));
    println!("Tweet Summary: {}", tweet.summarize());
    println!("Article Summary: {}", artical.summarize());
    println!("Article Summary: {}", return_summarization().summarize());
}
