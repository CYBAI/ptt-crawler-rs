extern crate reqwest;
extern crate select;

use reqwest::Client;
use select::document::Document;
use select::predicate::Class;

fn scrape_page(document: Document, keyword: &str) -> Vec<String> {
    document.find(Class("r-ent"))
                .filter_map(|node| node.find(Class("title")).next())
                .filter_map(|title| {
                    if title.text().trim().starts_with(&*format!("[{}]", keyword)) {
                        Some(title.text())
                    } else {
                        None
                    }
                })
                .map(|text| text.trim().to_string())
                .collect()
}

fn main() {
    let client = Client::new();
    let keyword = "售票";
    let board = "drama-ticket";
    let mut url = format!("http://ptt.cc/bbs/{}/index.html", board);
    let mut result: Vec<String> = vec![];

    while result.len() <= 20 {
        let response = match client.get(&*url).send() {
            Ok(res) => res,
            Err(_) => panic!("Failed to get html")
        };

        let document = Document::from_read(response);

        if let Ok(document) = document {
            let previous_pagenum = document.find(Class("wide")).nth(1);
            url = format!("http://ptt.cc{}", previous_pagenum.unwrap().attr("href").unwrap());

            result.append(&mut scrape_page(document.clone(), keyword));
        }
    }

    println!("{:?}", result);
}
