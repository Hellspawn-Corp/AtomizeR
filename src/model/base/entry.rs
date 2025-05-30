use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct BaseEntry {
    id: Uuid,
    title: String,
    url: String,
    published: DateTime<Utc>,
    updated: DateTime<Utc>,
    content: String,
    hash: String,
}

impl BaseEntry {
    pub fn new(
        id: Uuid,
        title: String,
        url: String,
        published: DateTime<Utc>,
        updated: DateTime<Utc>,
        content: String,
        hash: String,
    ) -> Self {
        BaseEntry {
            id,
            title,
            url,
            published,
            updated,
            content,
            hash,
        }
    }

    pub(crate) fn get_id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn get_title(&self) -> String {
        self.title.clone()
    }

    pub(crate) fn get_url(&self) -> String {
        self.url.clone()
    }

    pub(crate) fn get_published(&self) -> DateTime<Utc> {
        self.published
    }

    pub(crate) fn get_updated(&self) -> DateTime<Utc> {
        self.updated
    }

    pub(crate) fn get_content(&self) -> String {
        self.content.clone()
    }

    pub(crate) fn get_hash(&self) -> String {
        self.hash.clone()
    }
}
