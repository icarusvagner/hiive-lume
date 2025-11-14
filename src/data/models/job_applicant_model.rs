#[derive(Clone)]
pub struct JobApplicantModel {
    img_src: String,
    full_name: String,
    location: String,
    position: String,
}

impl JobApplicantModel {
    pub fn new(img_src: String, full_name: String, location: String, position: String) -> Self {
        Self {
            img_src,
            full_name,
            location,
            position,
        }
    }

    pub fn img_src(&self) -> String {
        self.img_src.clone()
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
