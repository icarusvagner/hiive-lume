#[derive(Clone)]
pub struct DashboardAttendanceOverviewModel {
    pub on_time: f64,
    pub late_arrival: f64,
    pub absent: f64,
    pub month: String,
}

impl DashboardAttendanceOverviewModel {
    pub fn new(on_time: f64, late_arrival: f64, absent: f64, month: String) -> Self {
        Self {
            on_time,
            late_arrival,
            absent,
            month,
        }
    }
}
