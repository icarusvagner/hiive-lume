use diesel::prelude::{Insertable, Queryable};
use lib_schema::schema::{tbl_address, tbl_employee};
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = tbl_address)]
pub struct AddressInsert {
	pub address_id: String,
	pub building_number: Option<String>,
	pub street_name: Option<String>,
	pub barangay: Option<String>,
	pub city: Option<String>,
	pub municipality: Option<String>,
	pub province: Option<String>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Employee {
	pub id: i32,
	pub employee_id: String,
	pub firstname: String,
	pub middlename: Option<String>,
	pub lastname: String,
	pub hire_date: chrono::DateTime<chrono::Utc>,
	pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = tbl_employee)]
pub struct EmployeeInsert {
	pub employee_id: String,
	pub firstname: String,
	pub middlename: Option<String>,
	pub lastname: String,
}
