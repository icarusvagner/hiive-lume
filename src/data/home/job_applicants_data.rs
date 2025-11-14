use mockd::{address, image, job, name};

use crate::data::models::job_applicant_model::JobApplicantModel;

pub struct JobApplicantData;

impl JobApplicantData {
    pub fn data() -> Vec<JobApplicantModel> {
        vec![
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
            JobApplicantModel::new(
                image::url(200, 200),
                name::full(),
                address::city(),
                job::title(),
            ),
        ]
    }
}
