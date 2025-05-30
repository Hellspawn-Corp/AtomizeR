use super::entry::BaseEntry;

trait Feed {
    fn get_title(&self) -> String;
    fn get_link(&self) -> String;
    fn get_updated(&self) -> String;
    fn get_id(&self) -> String;
    fn get_entries(&self) -> Vec<BaseEntry>;
}
