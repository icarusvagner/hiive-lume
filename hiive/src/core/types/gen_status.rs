use gpui::{Hsla, rgb};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GeneralStatus {
    Active,
    Inactive,
}

impl std::fmt::Display for GeneralStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Active => "Active",
                Self::Inactive => "Inactive",
            }
        )
    }
}

impl GeneralStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Inactive => "Inactive",
        }
    }

    pub fn color(&self) -> Hsla {
        match self {
            GeneralStatus::Active => rgb(0x28A745).into(),
            GeneralStatus::Inactive => rgb(0x6C757D).into(),
        }
    }
}
