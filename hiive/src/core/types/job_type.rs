#[derive(Debug, Clone)]
pub enum JobType {
    FullTime,
    PartTime,
    Temporary,
    Freelance,
    Contract,
}

impl std::fmt::Display for JobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JobType::FullTime => "Full Time",
                JobType::PartTime => "Part Time",
                JobType::Temporary => "Temporary",
                JobType::Freelance => "Freelance",
                JobType::Contract => "Contract",
            }
        )
    }
}

impl JobType {
    pub fn as_str(&self) -> &'static str {
        match self {
            JobType::FullTime => "Full Time",
            JobType::PartTime => "Part Time",
            JobType::Temporary => "Temporary",
            JobType::Freelance => "Freelance",
            JobType::Contract => "Contract",
        }
    }
}
