#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate reqwest;
extern crate regex;
extern crate select;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate rocket;

mod crawler;
mod ptt;

use reqwest::Client;
use crawler::Crawl;
use ptt::post::Post;
use select::document::Document;

use std::io::Cursor;
use rocket::response::Response;
use rocket::http::{ContentType, Status};

#[get("/post/<board_name>/<post_id>")]
fn search_post(board_name: String, post_id: String) -> Response<'static> {
    let url = format!("https://www.ptt.cc/bbs/{}/{}.html", board_name, post_id);
    let client = Client::new();
    if let Ok(response) = client.get(&url).send() {
        if let Ok(document) = Document::from_read(response) {
            let mut post_crawler = Post::new(post_id, board_name);
            if let Ok(post) = post_crawler.crawl(document) {
                let post_json = serde_json::to_string(&post).unwrap();

                return Response::build()
                        .status(Status::Ok)
                        .header(ContentType::JSON)
                        .sized_body(Cursor::new(post_json))
                        .finalize();
            }
        }
    };

    Response::build()
        .status(Status::new(500, "Document Parsing Error"))
        .header(ContentType::JSON)
        .finalize()
}

fn main() {
    rocket::ignite().mount("/search/", routes![search_post]).launch();
}
