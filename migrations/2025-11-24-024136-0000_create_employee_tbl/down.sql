-- This file should undo anything in `up.sql`
-- Master Tables
DROP TABLE IF EXISTS tbl_department;
DROP TABLE IF EXISTS tbl_salary_grade;
DROP TABLE IF EXISTS tbl_job_position;

-- Employee
DROP TABLE IF EXISTS tbl_address;
DROP TABLE IF EXISTS tbl_employee;

-- Time & Attendance
DROP TABLE IF EXISTS tbl_time_log;
DROP TABLE IF EXISTS tbl_attendance_record;

-- Leaves
DROP TABLE IF EXISTS tbl_leave_type;
DROP TABLE IF EXISTS tbl_leave_balance;
DROP TABLE IF EXISTS tbl_leave_request;

-- Payroll
DROP TABLE IF EXISTS tbl_earning;
DROP TABLE IF EXISTS tbl_deduction;
DROP TABLE IF EXISTS tbl_kpi;
DROP TABLE IF EXISTS tbl_payroll;
DROP TABLE IF EXISTS tbl_performance_review;
DROP TABLE IF EXISTS tbl_audit_log;
