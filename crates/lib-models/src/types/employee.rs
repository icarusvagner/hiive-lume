use diesel::expression::AsExpression;

#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	diesel::sql_types::SqlType,
	diesel::deserialize::FromSqlRow,
	serde::Serialize,
	serde::Deserialize,
)]
#[diesel(sql_type = employment_status)]
pub enum EmployementStatusType {
	FullTime,
	PartTime,
	Contract,
	Temporary,
	Seasonal,
	Intern,
	Apprentice,
	Probationary,
	Casual,
	ProjectBased,
}

impl ToString for EmployementStatusType {
	fn to_string(&self) -> String {
		match self {
			EmployementStatusType::FullTime => "full-time",
			EmployementStatusType::PartTime => "part-time",
			EmployementStatusType::Contract => "contract",
			EmployementStatusType::Temporary => "temporary",
			EmployementStatusType::Seasonal => "seasonal",
			EmployementStatusType::Intern => "intern",
			EmployementStatusType::Apprentice => "apprentice",
			EmployementStatusType::Probationary => "probationary",
			EmployementStatusType::Casual => "casual",
			EmployementStatusType::ProjectBased => "project based",
		}
		.to_string()
	}
}
