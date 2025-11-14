#[derive(Clone)]
pub struct JobApplicantModel {
    full_name: String,
    location: String,
    position: String,
}

impl JobApplicantModel {
    pub fn new(full_name: String, location: String, position: String) -> Self {
        Self {
            full_name,
            location,
            position,
        }
    }

    pub fn fullname(&self) -> String {
        self.full_name.clone()
    }

    pub fn location(&self) -> String {
        self.location.clone()
    }

    pub fn position(&self) -> String {
        self.position.clone()
    }
}
