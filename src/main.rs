extern crate reqwest;
extern crate regex;
extern crate select;
extern crate serde;
extern crate serde_json;

mod crawler;
mod ptt;

use reqwest::Client;
use crawler::Crawl;
use ptt::post::PostCrawler;
use select::document::Document;

fn main() {
    let url = "https://www.ptt.cc/bbs/Baseball/M.1526791990.A.35B.html";
    // User only has ID
    // let url = "https://www.ptt.cc/bbs/NBA/M.1526792335.A.DC5.html";

    let client = Client::new();

    let response = match client.get(url).send() {
        Ok(res) => res,
        Err(_) => panic!("Cannot found page info"),
    };

    let document = Document::from_read(response);
    if let Ok(document) = document {
        let post_crawler = PostCrawler::new("Baseball", "M.1526791990.A.35B");
        let doc = post_crawler.crawl(document);
        println!("{:?}", doc);
    }
}
