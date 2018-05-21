use select::document::Document;

pub trait Crawl {
    type Target;
    fn crawl(&mut self, document: Document) -> Result<Self::Target, ()>;
}
