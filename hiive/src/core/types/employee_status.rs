use gpui::{Hsla, rgb};

#[derive(Clone, Debug)]
pub enum EmployeeStatus {
    Active,
    Inactive,
}

impl std::fmt::Display for EmployeeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EmployeeStatus::Active => "Active",
                EmployeeStatus::Inactive => "Inactive",
            }
        )
    }
}

impl EmployeeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmployeeStatus::Active => "Active",
            EmployeeStatus::Inactive => "Inactive",
        }
    }

    pub fn color(&self) -> Hsla {
        match self {
            EmployeeStatus::Active => rgb(0x28A745).into(),
            EmployeeStatus::Inactive => rgb(0x6C757D).into(),
        }
    }
}
