#[derive(Debug, Clone)]
pub enum WorkType {
    Onsite,
    Remote,
    Hybrid,
}

impl std::fmt::Display for WorkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Onsite => "On-site",
                Self::Remote => "Remote",
                Self::Hybrid => "Hybrid",
            }
        )
    }
}

impl WorkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Onsite => "On-site",
            Self::Remote => "Remote",
            Self::Hybrid => "Hybrid",
        }
    }

    pub fn abbr(&self) -> &'static str {
        match self {
            Self::Onsite => "WFO",
            Self::Remote => "WFH",
            Self::Hybrid => "Hybrid",
        }
    }
}
