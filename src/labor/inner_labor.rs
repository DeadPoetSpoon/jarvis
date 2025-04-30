use std::collections::HashMap;

use super::Job;
use super::Labor;
use crate::JobKind;
use crate::Resource;
use crate::ResourceId;

pub struct InnerLabor {
    labor_vec: Vec<Box<dyn Labor>>,
}

impl Default for InnerLabor {
    fn default() -> Self {
        let msg_labor: Box<dyn Labor> = Box::new(InnerMsgLabor::default());
        Self {
            labor_vec: vec![msg_labor],
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub enum InnerJobKind {
    #[default]
    Nothing,
    AddMsg(Resource),
    GetMsg(Option<ResourceId>),
    RemoveMsg(Option<ResourceId>),
    HasMsg,
}
impl Labor for InnerLabor {
    fn handle(&mut self, job: &mut Job) -> anyhow::Result<Option<Vec<Job>>> {
        let mut return_job = Vec::new();
        match &job.kind {
            crate::JobKind::Inner(_) => {
                for labor in self.labor_vec.iter_mut() {
                    if let Some(mut job_vec) = labor.handle(job)? {
                        return_job.append(&mut job_vec);
                    };
                }
            }
            _ => {}
        };
        if return_job.iter().any(|_| true) {
            Ok(Some(return_job))
        } else {
            Ok(None)
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct InnerMsgLabor {
    msg_map: HashMap<ResourceId, Resource>,
}

impl Labor for InnerMsgLabor {
    fn handle(&mut self, job: &mut Job) -> anyhow::Result<Option<Vec<Job>>> {
        match &job.kind {
            JobKind::Inner(InnerJobKind::AddMsg(msg)) => {
                self.msg_map.insert(msg.id.clone(), msg.clone());
                job.finish();
            }
            JobKind::Inner(InnerJobKind::RemoveMsg(id)) => {
                if let Some(id) = id {
                    if self.msg_map.remove(id).is_none() {
                        job.chain_result(Resource::new_no_data());
                    } else {
                        job.chain_result(Resource::new_with_data());
                    }
                } else {
                    self.msg_map.clear();
                    job.chain_result(Resource::new_with_data());
                }
                job.finish();
            }
            JobKind::Inner(InnerJobKind::GetMsg(id)) => {
                if let Some(id) = id {
                    if let Some(msg) = self.msg_map.get(id) {
                        job.chain_result(msg.clone());
                    };
                } else if self.msg_map.iter().any(|_|true)  {
                    let result = Resource::new_mutli();
                    job.chain_result(result);
                    for item in self.msg_map.clone() {
                        job.chain_result(item.1);
                    }
                }else {
                    job.chain_result(Resource::new_no_data());
                }
                job.finish();
            }
            JobKind::Inner(InnerJobKind::HasMsg) => {
                if self.msg_map.iter().any(|_| true) {
                    job.chain_result(Resource::new_with_data());
                } else {
                    job.chain_result(Resource::new_no_data());
                }
                job.finish();
            }
            _ => {},
        };
        Ok(None)
    }
}
