use crate::data::models::events_news_model::EventsNewsModel;

pub struct EventsNewsData;

impl EventsNewsData {
    pub fn all_data() -> Vec<EventsNewsModel> {
        vec![
            EventsNewsModel::new(
                "2025-01-10 09:00:00".to_string(),
                "New Year Kick-Off Assembly".to_string(),
                "Company-wide meeting to discuss goals and priorities for 2025.".to_string(),
            ),
            EventsNewsModel::new(
                "2025-02-18 13:30:00".to_string(),
                "Board Meeting".to_string(),
                "All project managers and department heads are required to attend.".to_string(),
            ),
            EventsNewsModel::new(
                "2025-03-12 10:00:00".to_string(),
                "Employee Wellness Workshop".to_string(),
                "A one-day seminar on work-life balance and stress management.".to_string(),
            ),
            EventsNewsModel::new(
                "2025-04-05 22:00:00".to_string(),
                "IT Systems Maintenance".to_string(),
                "Scheduled downtime for security updates and infrastructure upgrades.".to_string(),
            ),
            EventsNewsModel::new(
                "2025-05-30 14:00:00".to_string(),
                "Quarterly Performance Review".to_string(),
                "Evaluation and reporting session for all departments.".to_string(),
            ),
            EventsNewsModel::new(
                "2025-06-14 08:30:00".to_string(),
                "Mid-Year Team Building".to_string(),
                "Outdoor activity promoting collaboration and camaraderie.".to_string(),
            ),
        ]
    }
}
