use std::fmt;

use gpui::{Hsla, rgb};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CandidateStatusType {
    Pending,
    Applied,
    InReview,
    Interviewing,
    Rejected,
    OfferExtended,
    Hired,
    Withdrawn,
    OnHold,
}

impl fmt::Display for CandidateStatusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CandidateStatusType::Pending => "Pending",
                CandidateStatusType::Applied => "Applied",
                CandidateStatusType::InReview => "In Review",
                CandidateStatusType::Interviewing => "Interviewing",
                CandidateStatusType::Rejected => "Rejected",
                CandidateStatusType::OfferExtended => "Offer Extended",
                CandidateStatusType::Hired => "Hired",
                CandidateStatusType::Withdrawn => "Withdrawn",
                CandidateStatusType::OnHold => "On Hold",
            }
        )
    }
}

impl CandidateStatusType {
    pub const ALL: [CandidateStatusType; 9] = [
        CandidateStatusType::Pending,
        CandidateStatusType::Applied,
        CandidateStatusType::InReview,
        CandidateStatusType::Interviewing,
        CandidateStatusType::Rejected,
        CandidateStatusType::OfferExtended,
        CandidateStatusType::Hired,
        CandidateStatusType::Withdrawn,
        CandidateStatusType::OnHold,
    ];

    pub const ALL_STR: [&'static str; 9] = [
        CandidateStatusType::Pending.as_str(),
        CandidateStatusType::Applied.as_str(),
        CandidateStatusType::InReview.as_str(),
        CandidateStatusType::Interviewing.as_str(),
        CandidateStatusType::Rejected.as_str(),
        CandidateStatusType::OfferExtended.as_str(),
        CandidateStatusType::Hired.as_str(),
        CandidateStatusType::Withdrawn.as_str(),
        CandidateStatusType::OnHold.as_str(),
    ];

    pub fn color(&self) -> Hsla {
        match self {
            CandidateStatusType::Pending => rgb(0xF5C757).into(),
            CandidateStatusType::Applied => rgb(0x4285F4).into(),
            CandidateStatusType::InReview => rgb(0x48658E).into(),
            CandidateStatusType::Interviewing => rgb(0x7B61FF).into(),
            CandidateStatusType::Rejected => rgb(0xEA4335).into(),
            CandidateStatusType::OfferExtended => rgb(0xFBBC05).into(),
            CandidateStatusType::Hired => rgb(0x34A853).into(),
            CandidateStatusType::Withdrawn => rgb(0x757575).into(),
            CandidateStatusType::OnHold => rgb(0xFF9800).into(),
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            CandidateStatusType::Pending => "Pending",
            CandidateStatusType::Applied => "Applied",
            CandidateStatusType::InReview => "In Review",
            CandidateStatusType::Interviewing => "Interviewing",
            CandidateStatusType::Rejected => "Rejected",
            CandidateStatusType::OfferExtended => "Offer Extended",
            CandidateStatusType::Hired => "Hired",
            CandidateStatusType::Withdrawn => "Withdrawn",
            CandidateStatusType::OnHold => "On Hold",
        }
    }
}
