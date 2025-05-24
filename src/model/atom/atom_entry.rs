struct AtomEntry {
    base: BaseEntry,
    summary: String,
}

impl Entry for AtomEntry {
    fn new(
        &self,
        id: Uuid,
        title: String,
        url: String,
        published: DateTime<Utc>,
        updated: DateTime<Utc>,
        content: String,
        hash: String,
        summary: String,
    ) -> Self {
        AtomEntry {
            entry: Entry::new(id, title, url, published, updated, content, hash),
            summary,
        }
    }

    fn new(
        &self,
        title: String,
        url: String,
        published: DateTime<Utc>,
        updated: DateTime<Utc>,
        content: String,
    ) -> Self {
        AtomEntry {
            entry: Entry::new(title, url, published, updated, content),
            summary,
        }
    }

    fn get_id(&self) -> Uuid {
        self.entry.get_id()
    }

    fn get_title(&self) -> String {
        self.entry.get_title()
    }

    fn get_url(&self) -> String {
        self.entry.get_url()
    }

    fn get_published(&self) -> DateTime<Utc> {
        self.entry.get_published()
    }

    fn get_updated(&self) -> DateTime<Utc> {
        self.entry.get_updated()
    }

    fn get_content(&self) -> String {
        self.entry.get_content()
    }

    fn get_hash(&self) -> String {
        self.entry.get_hash()
    }
}
