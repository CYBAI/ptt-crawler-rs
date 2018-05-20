use select::document::Document;

pub trait Crawl {
    type Target;
    fn crawl(&self, document: Document) -> Result<Self::Target, ()>;
}
