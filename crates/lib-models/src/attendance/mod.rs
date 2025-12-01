use diesel::prelude::Queryable;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct AttendanceRecord {
	pub id: i32,
	pub attendance_record_id: uuid::Uuid,
	pub employee_id: i32,
	pub date: chrono::DateTime<chrono::Utc>,
	pub cid: i32,
	pub ctime: chrono::DateTime<chrono::Utc>,
	pub mid: i32,
	pub mtime: chrono::DateTime<chrono::Utc>,
}
