use crate::data::models::attendance_overview_model::DashboardAttendanceOverviewModel;

#[derive(Clone)]
pub struct AttendanceOverviewData {}

impl AttendanceOverviewData {
    pub fn all_data() -> Vec<DashboardAttendanceOverviewModel> {
        vec![
            DashboardAttendanceOverviewModel::new(184.0, 10.0, 6.0, "Jan".to_string()),
            DashboardAttendanceOverviewModel::new(182.0, 12.0, 6.0, "Feb".to_string()),
            DashboardAttendanceOverviewModel::new(179.0, 15.0, 6.0, "Mar".to_string()),
            DashboardAttendanceOverviewModel::new(176.0, 17.0, 7.0, "Apr".to_string()),
            DashboardAttendanceOverviewModel::new(174.0, 18.0, 8.0, "May".to_string()),
            DashboardAttendanceOverviewModel::new(168.0, 22.0, 10.0, "Jun".to_string()),
            DashboardAttendanceOverviewModel::new(166.0, 24.0, 10.0, "Jul".to_string()),
            DashboardAttendanceOverviewModel::new(167.0, 23.0, 10.0, "Aug".to_string()),
            DashboardAttendanceOverviewModel::new(172.0, 20.0, 8.0, "Sep".to_string()),
            DashboardAttendanceOverviewModel::new(175.0, 18.0, 7.0, "Oct".to_string()),
            DashboardAttendanceOverviewModel::new(173.0, 17.0, 10.0, "Nov".to_string()),
            DashboardAttendanceOverviewModel::new(160.0, 20.0, 20.0, "Dec".to_string()),
        ]
    }
}
