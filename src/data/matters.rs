use chrono::{Local, NaiveDate};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Hash)]
pub struct Matters {
    pub name: String,
    pub des: Option<String>,
    pub start_time: NaiveDate,
    pub finish_time: Option<NaiveDate>,
    pub final_time: Option<NaiveDate>,
    pub magnitude: i8,
    pub urgency: i8,
    pub sub_matters: Vec<Matters>,
}

impl Default for Matters {
    fn default() -> Self {
        Self {
            name: Default::default(),
            des: Default::default(),
            start_time: Local::now().date_naive(),
            finish_time: Default::default(),
            final_time: Default::default(),
            magnitude: Default::default(),
            urgency: Default::default(),
            sub_matters: Default::default(),
        }
    }
}
