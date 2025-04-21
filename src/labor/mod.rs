mod inner_labor;
use std::collections::VecDeque;

pub use inner_labor::*;
use log::debug;

use crate::{InnerJobKind, Job, JobKind, Resource};

pub trait Labor {
    fn handle(&mut self, job: &mut Job) -> anyhow::Result<()>;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LaborHall {
    wait_job_queen: VecDeque<Job>,
    #[serde(skip)]
    run_job_queen: VecDeque<Job>,
    #[serde(skip)]
    finish_job_queen: VecDeque<Job>,
    #[serde(skip)]
    labor_vec: Vec<Box<dyn Labor>>,
    run_job_limit: usize,
}

impl Default for LaborHall {
    fn default() -> Self {
        let inner_labor: Box<InnerLabor> = Default::default();
        Self {
            wait_job_queen: Default::default(),
            run_job_queen: Default::default(),
            finish_job_queen: Default::default(),
            labor_vec: vec![inner_labor],
            run_job_limit: 8,
        }
    }
}

impl LaborHall {
    pub fn get_all_inner_msg(&mut self)-> anyhow::Result<Option<Resource>> {
        let mut job: Job = Default::default();
        job.kind(JobKind::Inner(InnerJobKind::GetMsg(None)));
        self.handle_job(&mut job)?;
        Ok(job.result)
    }
    pub fn labor_limit(&mut self, limit: usize) -> &mut Self {
        self.run_job_limit = limit;
        self
    }
    pub fn handle_job(&mut self, job: &mut Job) -> anyhow::Result<()> {
        for labor in self.labor_vec.iter_mut() {
            labor.handle(job)?
        }
        Ok(())
    }
    pub fn push_job(&mut self, job: Job) -> &mut Self {
        self.wait_job_queen.push_back(job);
        self
    }
    pub fn do_job(&mut self) -> anyhow::Result<()> {
        if let Some(mut job) = self.run_job_queen.pop_front() {
            if job.check_finish() {
                self.finish_job_queen.push_back(job);
            } else {
                self.run_job_queen.push_back(job);
            }
        };
        let len = self.run_job_queen.len();
        if len >= self.run_job_limit {
            return Ok(());
        }
        let job = self.wait_job_queen.pop_front();
        if job.is_none() {
            return Ok(());
        }
        let mut job = job.unwrap();
        if let Some(id) = &job.should_after {
            let has_finish = self.finish_job_queen.iter_mut().any(|x| x.is_me(id));
            if !has_finish {
                self.wait_job_queen.push_back(job);
                return Ok(());
            }
        }

        self.handle_job(&mut job)?;
        self.run_job_queen.push_back(job);
        Ok(())
    }
}
