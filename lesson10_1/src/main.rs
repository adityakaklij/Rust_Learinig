pub trait Summary {
    fn summarize(&self)-> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub content: String,
    pub auther: String,
    pub location:String,
}

pub struct Tweet {
    pub user: String,
    pub reply:String,
    pub time: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self)-> String{
        format!("{} {} {}", self.headline, self.auther, self.content);
    }
}

impl Sumarry for Tweet {
    fn summarize(&self)-> String {
        format!("{} {}", self.user, self.reply);
    }
}

fn main() {

}