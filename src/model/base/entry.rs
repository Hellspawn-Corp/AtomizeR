trait Entry {
    fn new(
        &self,
        id: Uuid,
        title: String,
        url: String,
        published: DateTime<Utc>,
        updated: DateTime<Utc>,
        content: String,
        hash: String,
    ) -> Self;
    fn new(
        &self,
        title: String,
        url: String,
        published: DateTime<Utc>,
        updated: DateTime<Utc>,
        content: String,
    ) -> Self;
    fn get_id(&self) -> Uuid;
    fn get_title(&self) -> String;
    fn get_url(&self) -> String;
    fn get_published(&self) -> DateTime<Utc>;
    fn get_updated(&self) -> DateTime<Utc>;
    fn get_content(&self) -> String;
    fn get_hash(&self) -> String;
}

struct BaseEntry {
    id: Uuid,
    title: String,
    url: String,
    published: DateTime<Utc>,
    updated: DateTime<Utc>,
    content: String,
    hash: String,
}

impl Entry for BaseEntry {
    fn new(
        &self,
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

    fn new(
        &self,
        title: String,
        url: String,
        published: DateTime<Utc>,
        updated: DateTime<Utc>,
        content: String,
    ) -> Self {
        BaseEntry {
            id: Uuid::new_v4(),
            title,
            url,
            published,
            updated,
            content,
            hash: String::new(),
        }
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_published(&self) -> DateTime<Utc> {
        self.published
    }

    fn get_updated(&self) -> DateTime<Utc> {
        self.updated
    }

    fn get_content(&self) -> String {
        self.content.clone()
    }

    fn get_hash(&self) -> String {
        self.hash.clone()
    }
}
