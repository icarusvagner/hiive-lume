use diesel::prelude::{Identifiable, Queryable};
use lib_schema::schema::tbl_applicant;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = tbl_applicant)]
pub struct Applicant {
	pub id: i32,
}
