use chrono::{DateTime, Local};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Matters {
    pub name: String,
    pub des: Option<String>,
    pub start_time: DateTime<Local>,
    pub finish_time: Option<DateTime<Local>>,
    pub final_time: Option<DateTime<Local>>,
    pub magnitude: i8,
    pub urgency: i8,
    pub sub_matters: Vec<Matters>,
}
