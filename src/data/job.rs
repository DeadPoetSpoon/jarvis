use std::collections::VecDeque;

use super::{job_kind::JobKind, Resource};
use chrono::{DateTime, Local};
use poll_promise::Promise;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Job {
    pub id: Uuid,
    pub kind: JobKind,
    pub start_time: DateTime<Local>,
    pub finish_time: Option<DateTime<Local>>,
    pub result: Option<Resource>,
    #[serde(skip)]
    pub promise_result_queen: VecDeque<Promise<Resource>>,
    pub should_after: Option<Uuid>,
}

impl Default for Job {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            kind: Default::default(),
            start_time: Default::default(),
            finish_time: Default::default(),
            result: Default::default(),
            promise_result_queen: Default::default(),
            should_after: Default::default(),
        }
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Job {
    pub fn is_me(&self, id: &Uuid) -> bool {
        self.id == *id
    }
    pub fn chain_result(&mut self, result: Resource) -> &mut Self {
        if self.result.is_none() {
            self.result(result)
        } else {
            let result_res = self.result.as_mut();
            result_res.unwrap().chain(result);
            self
        }
    }
    pub fn promise_result(&mut self, result: Promise<Resource>) -> &mut Self {
        self.promise_result_queen.push_back(result);
        self
    }
    fn result(&mut self, result: Resource) -> &mut Self {
        self.result = Some(result);
        self
    }
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            start_time: Local::now(),
            ..Default::default()
        }
    }
    pub fn should_after(&mut self, id: Uuid) -> &mut Self {
        self.should_after = Some(id);
        self
    }
    pub fn kind(&mut self, kind: JobKind) -> &mut Self {
        self.kind = kind;
        self
    }
    pub fn finish(&mut self) -> &mut Self {
        self.finish_time = Some(Local::now());
        self
    }
    pub fn is_finish(&mut self) -> bool {
        self.finish_time.is_some()
    }
    pub fn check_finish(&mut self) -> bool {
        let len = self.promise_result_queen.len();
        for _ in 0..len {
            if let Some(result) = self.promise_result_queen.pop_front() {
                if let Some(result) = result.ready() {
                    self.chain_result(result.to_owned());
                } else {
                    self.promise_result_queen.push_back(result);
                }
            };
        }
        let len = self.promise_result_queen.len();
        if len == 0 {
            self.finish();
        }
        self.is_finish()
    }
}
