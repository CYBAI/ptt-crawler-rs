use select::document::Document;
use select::predicate::{Class, Text};

use crawler::Crawl;
use ptt::comment::{CommentType, Comment};
use ptt::user::User;
use regex::Regex;

#[derive(Clone, Debug, Serialize)]
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
    pub fn new(id: String, board: String) -> Self {
        Post {
            id,
            author: User::new(String::from(""), None),
            title: String::from(""),
            time: String::from(""),
            content: String::from(""),
            comments: Vec::new(),
            ip: String::from(""),
            category: None,
            board,
        }
    }

    pub fn set_time(&mut self, time: &str) -> &mut Self {
        self.time = time.trim().to_string();

        self
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();

        self
    }

    pub fn set_ip(&mut self, document: &Document) -> &mut Self {
        self.ip = match document.find(Class("f2")).next() {
            Some(node) => {
                let ip_regex = Regex::new(r"(\d{0,4}\.\d{0,4}\.\d{0,4}\.\d{0,4})").unwrap();

                match ip_regex.captures(&*node.text()) {
                    Some(captured) => captured.get(0).unwrap().as_str().to_string(),
                    None => String::from("")
                }
            },
            None => String::from(""),
        };

        self
    }

    pub fn set_author(&mut self, user_line: &str) -> &mut Self {
        let user_regex = Regex::new(r"(?P<id>\w+)(?:\s\((?P<name>.*)\))?").unwrap();
        let captures = user_regex.captures(user_line);

        self.author = match captures {
            Some(caps) => User::new(
                caps.get(1).map_or(String::from(""), |m| String::from(m.as_str())),
                caps.get(2).and_then(|m| Some(String::from(m.as_str()))),
            ),
            None => User::new(String::from(""), None)
        };

        self
    }

    pub fn set_category(&mut self, title: &str) -> &mut Self {
        let category_regex = Regex::new(r"^\[(.*)\]").unwrap();
        self.category = match category_regex.captures(title) {
            Some(caps) => caps.get(1).and_then(|m| Some(String::from(m.as_str()))),
            None => None,
        };

        self
    }

    pub fn set_content(&mut self, document: &Document) -> &mut Self {
        self.content = document.find(Class("bbs-screen"))
                                .next()
                                .unwrap()
                                .children()
                                .filter_map(|n| {
                                    if n.is(Text) {
                                        Some(n)
                                    } else {
                                        None
                                    }
                                })
                                .nth(0)
                                .unwrap()
                                .text().trim().to_string();

        self
    }

    pub fn set_comments(&mut self, document: &Document) -> &mut Self {
        self.comments = document.find(Class("push"))
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
                                        Some(content) => content.text().trim().to_string(),
                                        None => String::from("")
                                    };

                                    let time = match node.find(Class("push-ipdatetime")).next() {
                                        Some(time) => time.text().trim().to_string(),
                                        None => String::from("")
                                    };

                                    Comment::new(type_, user, content, time)
                                })
                                .collect::<Vec<Comment>>();

        self
    }
}

impl Crawl for Post {
    type Target = Self;

    fn crawl(&mut self, document: Document) -> Result<Self, ()> {
        let metalines = document.find(Class("article-metaline"))
                                .filter_map(|n| n.find(Class("article-meta-value")).next())
                                .map(|n| n.text())
                                .collect::<Vec<String>>();

        self.set_author(&metalines[0])
            .set_title(&metalines[1])
            .set_category(&metalines[1])
            .set_time(&metalines[2])
            .set_ip(&document)
            .set_content(&document)
            .set_comments(&document);

        Ok(self.clone())
    }
}
