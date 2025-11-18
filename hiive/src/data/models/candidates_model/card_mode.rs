use crate::core::types::candidates_status::CandidateStatusType;

#[derive(Debug, Clone)]
pub struct CandidatesCardModel {
    src: String,
    fullname: String,
    position: String,
    status: CandidateStatusType,
    email: String,
    number: String,
    experience: String,
    date_applied: String,
}

impl CandidatesCardModel {
    pub fn new(
        src: String,
        fullname: String,
        position: String,
        status: CandidateStatusType,
        email: String,
        number: String,
        experience: String,
        date_applied: String,
    ) -> Self {
        Self {
            src,
            fullname,
            position,
            status,
            email,
            number,
            experience,
            date_applied,
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
    pub fn status(&self) -> CandidateStatusType {
        self.status.clone()
    }
    pub fn email(&self) -> String {
        self.email.clone()
    }
    pub fn number(&self) -> String {
        self.number.clone()
    }
    pub fn experience(&self) -> String {
        self.experience.clone()
    }
    pub fn date_applied(&self) -> String {
        self.date_applied.clone()
    }
}
