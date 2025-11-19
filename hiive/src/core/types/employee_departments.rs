#[derive(Clone, Debug)]
pub enum EmployeeDepartment {
    It,
    Hr,
    CallCenter,
}

impl std::fmt::Display for EmployeeDepartment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EmployeeDepartment::It => "IT",
                EmployeeDepartment::Hr => "HR",
                EmployeeDepartment::CallCenter => "Call Center",
            }
        )
    }
}

impl EmployeeDepartment {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmployeeDepartment::It => "IT",
            EmployeeDepartment::Hr => "HR",
            EmployeeDepartment::CallCenter => "Call Center",
        }
    }
}
