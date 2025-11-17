use crate::data::models::interviews_model::UpcomingInterviewModel;
use chrono::{Duration, Utc};

#[derive(Debug)]
pub struct InterviewsData;

impl InterviewsData {
    pub fn data() -> Vec<UpcomingInterviewModel> {
        let now = Utc::now();

        let entry = |fullname: &str, position: &str| {
            let start_time = now.time();
            let end_time = start_time + Duration::hours(3);

            UpcomingInterviewModel::new(
                "https://picsum.photos/500/500".into(),
                fullname.into(),
                position.into(),
                start_time.format("%H:%M").to_string(),
                end_time.format("%H:%M").to_string(),
            )
        };

        vec![
            entry("Daniel Cruz", "Backend Rust Developer"),
            entry("Marianne Lopez", "HR Assistant"),
            entry("Keanu Del Rosario", "Mobile App Developer"),
            entry("Alyssa Tan", "Frontend Engineer"),
            entry("Jerrod Villanueva", "QA Tester"),
            entry("Frances Dizon", "Project Coordinator"),
            entry("Miguel Soriano", "Systems Administrator"),
            entry("Trixie Manalili", "Data Analyst"),
            entry("Harvey Robles", "DevOps Engineer"),
            entry("Charlene Bautista", "UI/UX Designer"),
        ]
    }
}
