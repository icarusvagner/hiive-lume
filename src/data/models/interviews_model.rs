#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpcomingInterviewModel {
    src: String,
    fullname: String,
    position: String,
    time_start: String,
    time_end: String,
}

impl UpcomingInterviewModel {
    pub fn new(
        src: String,
        fullname: String,
        position: String,
        time_start: String,
        time_end: String,
    ) -> Self {
        Self {
            src,
            fullname,
            position,
            time_start,
            time_end,
        }
    }
    pub fn src(&self) -> String {
        self.src.clone()
    }
    pub fn fullname(&self) -> String {
        self.fullname.clone()
    }
    pub fn position(&self) -> String {
        self.position.clone()
    }
    pub fn time_start(&self) -> String {
        self.time_start.clone()
    }
    pub fn end_time(&self) -> String {
        self.time_end.clone()
    }
}
