use crate::core::types::{
    employee_departments::EmployeeDepartment, employee_status::EmployeeStatus,
};

#[derive(Clone, Debug)]
pub struct EmployeeModel {
    src: String,
    position: String,
    fullname: String,
    status: EmployeeStatus,
    email: String,
    number: String,
    department: EmployeeDepartment,
    date_join: String,
}

impl EmployeeModel {
    pub fn new(
        src: String,
        position: String,
        fullname: String,
        status: EmployeeStatus,
        email: String,
        number: String,
        department: EmployeeDepartment,
        date_join: String,
    ) -> Self {
        Self {
            src,
            position,
            fullname,
            status,
            email,
            number,
            department,
            date_join,
        }
    }

    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn position(&self) -> String {
        self.position.clone()
    }
    pub fn fullname(&self) -> String {
        self.fullname.clone()
    }
    pub fn status(&self) -> EmployeeStatus {
        self.status.clone()
    }
    pub fn email(&self) -> String {
        self.email.clone()
    }
    pub fn number(&self) -> String {
        self.number.clone()
    }
    pub fn department(&self) -> EmployeeDepartment {
        self.department.clone()
    }
    pub fn date_joined(&self) -> String {
        self.date_join.clone()
    }
}
