use crate::data::models::job_applicant_model::JobApplicantModel;

pub struct JobApplicantData;

impl JobApplicantData {
    pub fn data() -> Vec<JobApplicantModel> {
        vec![
            JobApplicantModel::new(
                "Elias Thompson".into(),
                "Cebu City, Philippines".into(),
                "Backend Rust Developer".into(),
            ),
            JobApplicantModel::new(
                "Mira Santos".into(),
                "Makati, Philippines".into(),
                "UI/UX Designer".into(),
            ),
            JobApplicantModel::new(
                "Caleb Fernandez".into(),
                "Davao City, Philippines".into(),
                "Full Stack Developer".into(),
            ),
            JobApplicantModel::new(
                "Hannah Lopez".into(),
                "Quezon City, Philippines".into(),
                "Project Manager".into(),
            ),
            JobApplicantModel::new(
                "Jerome Castillo".into(),
                "Manila, Philippines".into(),
                "DevOps Engineer".into(),
            ),
            JobApplicantModel::new(
                "Sophia Reyes".into(),
                "Iloilo City, Philippines".into(),
                "Frontend Developer".into(),
            ),
            JobApplicantModel::new(
                "Noah Garcia".into(),
                "Taguig, Philippines".into(),
                "Mobile App Developer".into(),
            ),
            JobApplicantModel::new(
                "Aubrey Lim".into(),
                "Baguio City, Philippines".into(),
                "Quality Assurance Tester".into(),
            ),
            JobApplicantModel::new(
                "Gabriel Ramirez".into(),
                "Cagayan de Oro, Philippines".into(),
                "Data Analyst".into(),
            ),
            JobApplicantModel::new(
                "Kyla Mendoza".into(),
                "General Santos, Philippines".into(),
                "Human Resource Assistant".into(),
            ),
        ]
    }
}
