use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::base::entry::BaseEntry;

pub struct AtomEntry {
    base: BaseEntry,
    summary: String,
}

impl AtomEntry {
    fn new(
        id: Uuid,
        title: String,
        url: String,
        published: DateTime<Utc>,
        updated: DateTime<Utc>,
        content: String,
        summary: String,
        hash: String,
    ) -> Self {
        AtomEntry {
            base: BaseEntry::new(id, title, url, published, updated, content, hash),
            summary,
        }
    }

    fn get_id(&self) -> Uuid {
        self.base.get_id()
    }

    fn get_title(&self) -> String {
        self.base.get_title()
    }

    fn get_url(&self) -> String {
        self.base.get_url()
    }

    fn get_published(&self) -> DateTime<Utc> {
        self.base.get_published()
    }

    fn get_updated(&self) -> DateTime<Utc> {
        self.base.get_updated()
    }

    fn get_content(&self) -> String {
        self.base.get_content()
    }

    fn get_hash(&self) -> String {
        self.base.get_hash()
    }
}
