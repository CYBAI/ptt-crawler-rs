use select::document::Document;
use select::predicate::{Class, Text};

use crawler::Crawl;
use ptt::comment::{CommentType, Comment};
use ptt::user::User;
use regex::Regex;

#[derive(Debug)]
pub struct Post {
    id: String,
    author: User,
    title: String,
    time: String,
    content: String,
    comments: Vec<Comment>,
    ip: String,
    category: Option<String>,
    board: String,
}

impl Post {
    pub fn new(
        id: String,
        author: User,
        title: String,
        time: String,
        content: String,
        comments: Vec<Comment>,
        ip: String,
        category: Option<String>,
        board: String,
    ) -> Self {
        Post {
            id,
            author,
            title,
            time,
            content,
            comments,
            ip,
            category,
            board,
        }
    }
}

pub struct PostCrawler {
    id: String,
    board: String,
}

impl PostCrawler {
    pub fn new(id: &str, board: &str) -> Self {
        Self {
            id: id.to_string(),
            board: board.to_string()
        }
    }
}

impl Crawl for PostCrawler {
    type Target = Post;

    fn crawl(&self, document: Document) -> Result<Post, ()> {
        let metalines = document.find(Class("article-metaline"))
                                .filter_map(|n| n.find(Class("article-meta-value")).next())
                                .map(|n| n.text())
                                .collect::<Vec<String>>();

        let category_regex = Regex::new(r"^\[(.*)\]").unwrap();
        let category = match category_regex.captures(&*metalines[1]) {
            Some(caps) => caps.get(1).and_then(|m| Some(String::from(m.as_str()))),
            None => None,
        };

        let content = document.find(Class("bbs-screen"))
                                .next()
                                .unwrap()
                                .children()
                                .filter_map(|n| {
                                    if n.is(Text) {
                                        Some(n)
                                    } else {
                                        None
                                    }
                                }).nth(0).unwrap();

        let comments = document.find(Class("push"))
                                .map(|node| {
                                    let type_ = match node.find(Class("push-tag")).next() {
                                        Some(push_tag) => {
                                            match push_tag.text().trim() {
                                                "噓" => CommentType::Bad,
                                                "推" => CommentType::Good,
                                                _ => CommentType::Normal,
                                            }
                                        },
                                        None => CommentType::Normal,
                                    };

                                    let user = match node.find(Class("push-userid")).next() {
                                        Some(id) => User::new(id.text(), None),
                                        None => User::new(String::from(""), None)
                                    };

                                    let content = match node.find(Class("push-content")).next() {
                                        Some(content) => content.text(),
                                        None => String::from("")
                                    };

                                    let time = match node.find(Class("push-ipdatetime")).next() {
                                        Some(time) => time.text(),
                                        None => String::from("")
                                    };

                                    Comment::new(type_, user, content, time)
                                })
                                .collect::<Vec<Comment>>();

        let user_regex = Regex::new(r"(?P<id>\w+)(?:\s\((?P<name>.*)\))?").unwrap();
        let captures = user_regex.captures(&*metalines[0]);

        let author = match captures {
            Some(caps) => User::new(
                caps.get(1).map_or(String::from(""), |m| String::from(m.as_str())),
                caps.get(2).and_then(|m| Some(String::from(m.as_str()))),
            ),
            None => return Err(())
        };

        let ip = match document.find(Class("f2")).next() {
            Some(node) => {
                let ip_regex = Regex::new(r"(\d{0,4}\.\d{0,4}\.\d{0,4}\.\d{0,4})").unwrap();

                match ip_regex.captures(&*node.text()) {
                    Some(captured) => captured.get(0).unwrap().as_str().to_string(),
                    None => String::from("")
                }
            },
            None => String::from(""),
        };

        Ok(Post::new(
            self.id.clone(),
            author,
            metalines[1].clone(),
            metalines[2].clone(),
            content.text().trim().to_string(),
            comments,
            ip,
            category,
            self.board.clone(),
        ))
    }
}
