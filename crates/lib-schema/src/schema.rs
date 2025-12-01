// @generated automatically by Diesel CLI.

pub mod sql_types {
	#[derive(
		diesel::query_builder::QueryId,
		Clone,
		std::fmt::Debug,
		diesel::sql_types::SqlType,
	)]
	#[diesel(postgres_type(name = "applicant_status"))]
	pub struct ApplicantStatus;

	#[derive(
		diesel::query_builder::QueryId,
		Clone,
		std::fmt::Debug,
		diesel::sql_types::SqlType,
	)]
	#[diesel(postgres_type(name = "attendance_status"))]
	pub struct AttendanceStatus;

	#[derive(
		diesel::query_builder::QueryId,
		Clone,
		std::fmt::Debug,
		diesel::sql_types::SqlType,
	)]
	#[diesel(postgres_type(name = "leave_request_status"))]
	pub struct LeaveRequestStatus;

	#[derive(
		diesel::query_builder::QueryId,
		Clone,
		std::fmt::Debug,
		diesel::sql_types::SqlType,
	)]
	#[diesel(postgres_type(name = "performance_review_status"))]
	pub struct PerformanceReviewStatus;

	#[derive(
		diesel::query_builder::QueryId,
		Clone,
		std::fmt::Debug,
		diesel::sql_types::SqlType,
	)]
	#[diesel(postgres_type(name = "user_session_state"))]
	pub struct UserSessionState;
}

diesel::table! {
	tbl_address (id) {
		id -> Int8,
		#[max_length = 120]
		address_id -> Varchar,
		building_number -> Nullable<Text>,
		street_name -> Nullable<Text>,
		barangay -> Nullable<Text>,
		city -> Nullable<Text>,
		municipality -> Nullable<Text>,
		province -> Nullable<Text>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	use diesel::sql_types::*;
	use super::sql_types::ApplicantStatus;

	tbl_applicant (id) {
		id -> Int8,
		#[max_length = 150]
		full_name -> Varchar,
		#[max_length = 150]
		email -> Varchar,
		status -> ApplicantStatus,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	use diesel::sql_types::*;
	use super::sql_types::AttendanceStatus;

	tbl_attendance_record (id) {
		id -> Int8,
		attendance_record_id -> Uuid,
		employee_id -> Int8,
		date -> Date,
		status -> AttendanceStatus,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_audit_log (id) {
		id -> Int8,
		actor_id -> Nullable<Int8>,
		actor_role -> Nullable<Text>,
		action -> Text,
		event_type -> Text,
		severity -> Text,
		target_table -> Text,
		target_id -> Nullable<Int8>,
		target_summary -> Nullable<Text>,
		old_values -> Nullable<Jsonb>,
		new_values -> Nullable<Jsonb>,
		diff -> Nullable<Jsonb>,
		change_source -> Nullable<Text>,
		ip_address -> Nullable<Inet>,
		user_agent -> Nullable<Text>,
		device_info -> Nullable<Jsonb>,
		trace_id -> Nullable<Text>,
		session_id -> Nullable<Uuid>,
		metadata -> Nullable<Jsonb>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_deduction (id) {
		id -> Int8,
		#[max_length = 120]
		deduction_id -> Varchar,
		#[max_length = 120]
		name -> Varchar,
		amount -> Nullable<Numeric>,
		percentage -> Nullable<Numeric>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_department (id) {
		id -> Int8,
		#[max_length = 120]
		department_id -> Varchar,
		name -> Text,
		description -> Nullable<Text>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_earning (id) {
		id -> Int8,
		#[max_length = 120]
		earning_id -> Varchar,
		#[max_length = 120]
		name -> Varchar,
		amount -> Numeric,
		taxable -> Nullable<Bool>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	use diesel::sql_types::*;
	use super::sql_types::UserSessionState;

	tbl_employee (id) {
		id -> Int8,
		#[max_length = 120]
		employee_id -> Varchar,
		#[max_length = 120]
		firstname -> Varchar,
		middlename -> Nullable<Text>,
		#[max_length = 120]
		lastname -> Varchar,
		hire_date -> Date,
		status -> UserSessionState,
		supervisor_id -> Nullable<Int8>,
		address_id -> Nullable<Int8>,
		department_id -> Nullable<Int8>,
		job_position_id -> Nullable<Int8>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_job_position (id) {
		id -> Int8,
		#[max_length = 120]
		job_position_id -> Varchar,
		title -> Text,
		department_id -> Int8,
		salary_grade_id -> Int8,
		description -> Nullable<Text>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_kpi (id) {
		id -> Int8,
		#[max_length = 120]
		kpi_id -> Varchar,
		#[max_length = 120]
		name -> Varchar,
		weight -> Nullable<Int4>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_leave_balance (id) {
		id -> Int8,
		employee_id -> Int8,
		leave_type_id -> Int8,
		balance_days -> Int4,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	use diesel::sql_types::*;
	use super::sql_types::LeaveRequestStatus;

	tbl_leave_request (id) {
		id -> Int8,
		employee_id -> Int8,
		leave_type_id -> Int8,
		start_date -> Date,
		end_date -> Date,
		reason -> Nullable<Text>,
		status -> LeaveRequestStatus,
		created_at -> Nullable<Timestamp>,
		updated_at -> Nullable<Timestamp>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_leave_type (id) {
		id -> Int8,
		leave_type_id -> Uuid,
		name -> Text,
		max_days -> Nullable<Int4>,
		is_paid -> Nullable<Bool>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_payroll (id) {
		id -> Int8,
		employee_id -> Int8,
		start_period -> Date,
		end_period -> Date,
		gross_pay -> Numeric,
		total_deductions -> Nullable<Numeric>,
		net_pay -> Numeric,
		date_processed -> Nullable<Timestamp>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	use diesel::sql_types::*;
	use super::sql_types::PerformanceReviewStatus;

	tbl_performance_review (id) {
		id -> Int8,
		employee_id -> Int8,
		date_evaluated -> Date,
		score -> Nullable<Int4>,
		remarks -> Nullable<Text>,
		status -> Nullable<PerformanceReviewStatus>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_permission (id) {
		id -> Int8,
		module -> Text,
		action -> Text,
		level -> Nullable<Text>,
		cid -> Int8,
		ctime -> Timestamptz,
	}
}

diesel::table! {
	tbl_role (id) {
		id -> Int8,
		name -> Text,
		description -> Nullable<Text>,
		cid -> Int8,
		ctime -> Timestamptz,
	}
}

diesel::table! {
	tbl_role_permission (id) {
		id -> Int8,
		role_id -> Nullable<Int8>,
		permission_id -> Nullable<Int8>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_salary_grade (id) {
		id -> Int8,
		#[max_length = 120]
		salary_grade_id -> Varchar,
		level -> Text,
		base_salary -> Numeric,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_time_log (id) {
		id -> Int8,
		time_log_id -> Uuid,
		employee_id -> Int8,
		check_in -> Timestamp,
		check_out -> Nullable<Timestamp>,
		notes -> Nullable<Text>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_user_account (id) {
		id -> Int8,
		#[max_length = 120]
		user_id -> Varchar,
		employee_id -> Nullable<Int8>,
		#[max_length = 128]
		username -> Varchar,
		#[max_length = 128]
		password_hash -> Nullable<Varchar>,
		pass_salt -> Uuid,
		token_salt -> Uuid,
		status -> Nullable<Text>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::table! {
	tbl_user_role (id) {
		id -> Int8,
		user_id -> Nullable<Int8>,
		role_id -> Nullable<Int8>,
		cid -> Int8,
		ctime -> Timestamptz,
		mid -> Int8,
		mtime -> Timestamptz,
	}
}

diesel::joinable!(tbl_attendance_record -> tbl_employee (employee_id));
diesel::joinable!(tbl_audit_log -> tbl_user_account (actor_id));
diesel::joinable!(tbl_employee -> tbl_address (address_id));
diesel::joinable!(tbl_employee -> tbl_department (department_id));
diesel::joinable!(tbl_employee -> tbl_job_position (job_position_id));
diesel::joinable!(tbl_job_position -> tbl_department (department_id));
diesel::joinable!(tbl_job_position -> tbl_salary_grade (salary_grade_id));
diesel::joinable!(tbl_leave_balance -> tbl_employee (employee_id));
diesel::joinable!(tbl_leave_balance -> tbl_leave_type (leave_type_id));
diesel::joinable!(tbl_leave_request -> tbl_employee (employee_id));
diesel::joinable!(tbl_leave_request -> tbl_leave_type (leave_type_id));
diesel::joinable!(tbl_payroll -> tbl_employee (employee_id));
diesel::joinable!(tbl_performance_review -> tbl_employee (employee_id));
diesel::joinable!(tbl_role_permission -> tbl_permission (permission_id));
diesel::joinable!(tbl_role_permission -> tbl_role (role_id));
diesel::joinable!(tbl_time_log -> tbl_employee (employee_id));
diesel::joinable!(tbl_user_account -> tbl_employee (employee_id));
diesel::joinable!(tbl_user_role -> tbl_role (role_id));
diesel::joinable!(tbl_user_role -> tbl_user_account (user_id));

diesel::allow_tables_to_appear_in_same_query!(
	tbl_address,
	tbl_applicant,
	tbl_attendance_record,
	tbl_audit_log,
	tbl_deduction,
	tbl_department,
	tbl_earning,
	tbl_employee,
	tbl_job_position,
	tbl_kpi,
	tbl_leave_balance,
	tbl_leave_request,
	tbl_leave_type,
	tbl_payroll,
	tbl_performance_review,
	tbl_permission,
	tbl_role,
	tbl_role_permission,
	tbl_salary_grade,
	tbl_time_log,
	tbl_user_account,
	tbl_user_role,
);
