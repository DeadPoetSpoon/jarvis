use std::collections::HashMap;

use super::Job;
use super::Labor;
use crate::InnerJobKind;
use crate::JobKind;
use crate::Resource;
use crate::ResourceId;

pub struct InnerLabor {
    labor_map: Vec<Box<dyn Labor>>,
}

impl Default for InnerLabor {
    fn default() -> Self {
        let msg_labor: Box<dyn Labor> = Box::new(InnerMsgLabor::default());
        Self {
            labor_map: vec![msg_labor],
        }
    }
}

impl Labor for InnerLabor {
    fn handle(&mut self, job: &mut Job) -> anyhow::Result<()> {
        match &job.kind {
            crate::JobKind::Inner(_) => {
                for porter in self.labor_map.iter_mut() {
                    porter.handle(job)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct InnerMsgLabor {
    msg_map: HashMap<ResourceId, Resource>,
}

impl Labor for InnerMsgLabor {
    fn handle(&mut self, job: &mut Job) -> anyhow::Result<()> {
        match &job.kind {
            JobKind::Inner(InnerJobKind::AddMsg(msg)) => {
                self.msg_map.insert(msg.id.clone(), msg.clone());
                job.finish();
                Ok(())
            }
            JobKind::Inner(InnerJobKind::RemoveMsg(id))=>{
                if let Some(id) = id {
                    if self.msg_map.remove(id).is_none() {
                        job.chain_result(Resource::new_no_data());
                    }else {
                        job.chain_result(Resource::new_with_data()) ;                       
                    }
                }else {
                    self.msg_map.clear();
                    job.chain_result(Resource::new_with_data()) ;
                }
                job.finish();
                Ok(())
            }
            JobKind::Inner(InnerJobKind::GetMsg(id)) => {
                if let Some(id) = id {
                    if let Some(msg) = self.msg_map.get(id) {
                        job.chain_result(msg.clone());
                    };
                } else if self.msg_map.len() > 0 {
                    let result = Resource::mutli();
                    job.chain_result(result);
                    for item in self.msg_map.clone() {
                        job.chain_result(item.1);
                    }
                }
                job.finish();
                Ok(())
            }
            JobKind::Inner(InnerJobKind::HasMsg) => {
                if self.msg_map.iter().any(|_|true) {
                   job.chain_result(Resource::new_with_data());
                }else {
                   job.chain_result(Resource::new_no_data());
                }
                job.finish();
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
