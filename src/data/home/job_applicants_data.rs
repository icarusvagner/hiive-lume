use crate::data::models::job_applicant_model::JobApplicantModel;

pub struct JobApplicantData;

impl JobApplicantData {
    pub fn data() -> Vec<JobApplicantModel> {
        vec![
            JobApplicantModel::new(
                "https://randomuser.me/api/portraits/men/11.jpg".into(),
                "Elias Thompson".into(),
                "Cebu City, Philippines".into(),
                "Backend Rust Developer".into(),
            ),
            JobApplicantModel::new(
                "https://randomuser.me/api/portraits/women/21.jpg".into(),
                "Mira Santos".into(),
                "Makati, Philippines".into(),
                "UI/UX Designer".into(),
            ),
            JobApplicantModel::new(
                "https://randomuser.me/api/portraits/men/54.jpg".into(),
                "Caleb Fernandez".into(),
                "Davao City, Philippines".into(),
                "Full Stack Developer".into(),
            ),
            JobApplicantModel::new(
                "https://randomuser.me/api/portraits/women/65.jpg".into(),
                "Hannah Lopez".into(),
                "Quezon City, Philippines".into(),
                "Project Manager".into(),
            ),
            JobApplicantModel::new(
                "https://randomuser.me/api/portraits/men/77.jpg".into(),
                "Jerome Castillo".into(),
                "Manila, Philippines".into(),
                "DevOps Engineer".into(),
            ),
            JobApplicantModel::new(
                "https://randomuser.me/api/portraits/women/8.jpg".into(),
                "Sophia Reyes".into(),
                "Iloilo City, Philippines".into(),
                "Frontend Developer".into(),
            ),
            JobApplicantModel::new(
                "https://randomuser.me/api/portraits/men/31.jpg".into(),
                "Noah Garcia".into(),
                "Taguig, Philippines".into(),
                "Mobile App Developer".into(),
            ),
            JobApplicantModel::new(
                "https://randomuser.me/api/portraits/women/33.jpg".into(),
                "Aubrey Lim".into(),
                "Baguio City, Philippines".into(),
                "Quality Assurance Tester".into(),
            ),
            JobApplicantModel::new(
                "https://randomuser.me/api/portraits/men/29.jpg".into(),
                "Gabriel Ramirez".into(),
                "Cagayan de Oro, Philippines".into(),
                "Data Analyst".into(),
            ),
            JobApplicantModel::new(
                "https://randomuser.me/api/portraits/women/19.jpg".into(),
                "Kyla Mendoza".into(),
                "General Santos, Philippines".into(),
                "Human Resource Assistant".into(),
            ),
        ]
    }
}
