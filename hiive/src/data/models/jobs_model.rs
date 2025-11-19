use crate::core::types::{gen_status::GeneralStatus, job_type::JobType, work_type::WorkType};

#[derive(Debug, Clone)]
pub struct JobsModel {
    image: String,
    title: String,
    desc: String,
    available_pos: u32,
    job_type: JobType,
    work_type: WorkType,
    job_status: GeneralStatus,
}

impl JobsModel {
    pub fn new(
        image: String,
        title: String,
        desc: String,
        available_pos: u32, // Positions available
        job_type: JobType,
        work_type: WorkType,
        job_status: GeneralStatus,
    ) -> Self {
        Self {
            image,
            title,
            desc,
            available_pos,
            job_type,
            work_type,
            job_status,
        }
    }

    pub fn image(&self) -> String {
        self.image.clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn description(&self) -> String {
        self.desc.clone()
    }
    pub fn available_position(&self) -> u32 {
        self.available_pos.clone()
    }
    pub fn job_type(&self) -> JobType {
        self.job_type.clone()
    }
    pub fn work_type(&self) -> WorkType {
        self.work_type.clone()
    }
    pub fn job_status(&self) -> GeneralStatus {
        self.job_status.clone()
    }
}
