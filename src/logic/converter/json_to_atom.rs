use chrono::{DateTime, FixedOffset, Utc};

// fn main() {
//     let dt: DateTime<FixedOffset> = DateTime::parse_from_rfc2822("").unwrap();
//     let atom_time = dt.to_rfc3339();
//     println!("{}", atom_time);

//     // Example Atom entry
//     let atom_entry = format!("<entry>\n  <updated>{}</updated>\n</entry>", atom_time);
//     println!("{}", atom_entry);
// }

use crate::model::json::json_entry::JsonEntry;
use atom_syndication::{Content, Entry, Feed};

pub fn json_entries_to_atom(posts: &[JsonEntry]) -> Feed {
    let updated = posts
        .iter()
        .map(|p| p.published)
        .max()
        .unwrap_or_else(|| Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()));

    let entries: Vec<Entry> = posts
        .iter()
        .map(|post| {
            let mut entry = Entry::default();
            entry.set_id(format!("tag:example.com,2025:{}", post.id));
            entry.set_title(post.title.clone());
            entry.set_updated(post.published);
            let mut content = Content::default();
            content.set_content_type(Some("html".to_string()));
            content.set_value(Some(post.content.clone()));
            entry.set_content(Some(content));
            entry
        })
        .collect();

    let mut feed = Feed::default();
    feed.set_title("Example Feed".to_string());
    feed.set_id("tag:example.com,2025:feed".to_string());
    feed.set_updated(updated);
    feed.set_entries(entries);
    feed
}
