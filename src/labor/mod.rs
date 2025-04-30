mod inner_labor;
mod schedule_labor;
mod storage_labor;
use std::{collections::VecDeque, ops::DerefMut};

pub use inner_labor::*;
pub use schedule_labor::*;
pub use storage_labor::*;

use crate::{Job, JobKind, Resource};

pub trait Labor {
    fn handle(&mut self, job: &mut Job) -> anyhow::Result<Option<Vec<Job>>>;
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
    pub fn clear_all_inner_msg(&mut self) -> anyhow::Result<()> {
        self.handle_job_by_kind(JobKind::remove_inner_msg(None))?;
        Ok(())
    }
    pub fn has_inner_msg(&mut self) -> anyhow::Result<bool> {
        let r= match self.handle_job_by_kind(JobKind::has_inner_msg())? {
            Some(res) => res.has_data(),
            None => false,
        };
        Ok(r)
    }
    pub fn get_all_inner_msg(&mut self)-> anyhow::Result<Option<Resource>> {
        self.handle_job_by_kind(JobKind::get_inner_msg(None))
    }
    pub fn labor_limit(&mut self, limit: usize) -> &mut Self {
        self.run_job_limit = limit;
        self
    }
    fn handle_job_by_kind(&mut self,kind:JobKind) -> anyhow::Result<Option<Resource>> {
        let mut job:Job = Default::default();
        job.kind(kind);
        self.handle_job(&mut job)?;
        Ok(job.result)
    }
    fn handle_job(&mut self, job: &mut Job) -> anyhow::Result<()> {
        let mut return_job_vec = Vec::new();
        for labor in self.labor_vec.iter_mut() {
            if let Some(mut job_vec) = labor.handle(job)? {
                return_job_vec.append(&mut job_vec);
            }
        }
        self.push_job_vec(return_job_vec);
        Ok(())
    }
    pub fn push_job(&mut self, job: Job) -> &mut Self {
        self.wait_job_queen.push_back(job);
        self
    }
    pub fn push_job_vec(&mut self,job_vec:Vec<Job>) -> &mut Self {
        for job in job_vec {
            self.push_job(job);
        }
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
