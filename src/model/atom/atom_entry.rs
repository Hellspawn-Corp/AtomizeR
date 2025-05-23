impl Entry for AtomEntry {
    fn new(title: String, url: String, summary: String, updated: String) -> Self {
        AtomEntry {
            title,
            url,
            summary,
            updated,
        }
    }
}
