pub struct Playlist {
    pub list: Vec<Animation>,
    pub sort: SortType
}

pub enum SortType {
    Unsorted,
    Alphabetical
}
