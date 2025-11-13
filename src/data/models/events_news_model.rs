pub struct EventsNewsModel {
    pub date_time: String,
    pub title: String,
    pub short_desc: String,
}

impl EventsNewsModel {
    pub fn new(date_time: String, title: String, short_desc: String) -> Self {
        Self {
            date_time,
            title,
            short_desc,
        }
    }
}
