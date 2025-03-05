use std::path::PathBuf;

use chrono::{DateTime, Datelike, Local};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub father: Option<Uuid>,
    pub des: Option<String>,
    pub start_time: DateTime<Local>,
    pub finish_time: Option<DateTime<Local>>,
    pub final_time: Option<DateTime<Local>>,
    pub magnitude: i8,
    pub urgency: i8,
}

impl Job {
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        let start_time = Local::now();
        Self {
            id,
            start_time,
            ..Default::default()
        }
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn make_sub(&mut self, father_id: Uuid) -> &mut Self {
        self.father = Some(father_id);
        self
    }
    pub fn set_des(&mut self, des: String) -> &mut Self {
        self.des = Some(des);
        self
    }
    pub fn finish(&mut self) -> &mut Self {
        self.finish_time = Some(Local::now());
        self
    }
    pub fn set_final_time(&mut self, final_time: DateTime<Local>) -> &mut Self {
        self.final_time = Some(final_time);
        self
    }
    pub fn remove_final_time(&mut self) -> &mut Self {
        self.final_time = None;
        self
    }
    pub fn set_magnitude(&mut self, magnitude: i8) -> &mut Self {
        self.magnitude = magnitude;
        self
    }
    pub fn set_urgency(&mut self, urgency: i8) -> &mut Self {
        self.urgency = urgency;
        self
    }
    pub fn is_father(&self) -> bool {
        self.father.is_none()
    }
    pub fn path(&self) -> PathBuf {
        let mut path = PathBuf::from("job");
        let start_time = self.start_time;
        path = path.join(PathBuf::from(start_time.year().to_string()));
        path = path.join(PathBuf::from(start_time.month().to_string()));
        let filename = format!("{}.job", self.id);
        path = path.join(PathBuf::from(filename));
        path
    }
}
