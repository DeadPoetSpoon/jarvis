use std::{
    collections::{BTreeMap, HashMap},
    net::IpAddr,
    path::PathBuf,
};

use chrono::format::Item;

use crate::{Resource, ResourceId};

use super::Labor;

#[derive(Default)]
pub enum StorageLaborKind {
    #[default]
    InMemary,
    Local(PathBuf),
    Dufs(IpAddr, u32),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub enum StorageJobKind {
    #[default]
    Nothing,
    Get(Option<ResourceId>),
    Save(Resource),
    Delete(Option<ResourceId>),
    Search(Option<String>, Option<String>),
}

pub struct InMemaryStorageLabor {
    storage: HashMap<ResourceId, Resource>,
}

impl Labor for InMemaryStorageLabor {
    fn handle(&mut self, job: &mut crate::Job) -> anyhow::Result<Option<Vec<crate::Job>>> {
        let kind = match &job.kind {
            crate::JobKind::Storage(x) => Some(x),
            _ => None,
        };
        if kind.is_none() {
            return Ok(None);
        }
        let kind = kind.unwrap();
        match kind {
            StorageJobKind::Nothing => {}
            StorageJobKind::Get(resource_id) => {
                if let Some(id) = resource_id {
                    if let Some(res) = self.storage.get(id) {
                        job.chain_result(res.clone());
                    }
                } else if self.storage.iter().any(|_| true) {
                    let result = Resource::new_mutli();
                    job.chain_result(result);
                    for item in self.storage.clone() {
                        job.chain_result(item.1);
                    }
                } else {
                    job.chain_result(Resource::new_no_data());
                }
                job.finish();
            }
            StorageJobKind::Save(resource) => {
                self.storage.insert(resource.id.clone(), resource.clone());
                job.finish();
            }
            StorageJobKind::Delete(resource_id) => {
                if let Some(id) = resource_id {
                    if self.storage.remove(id).is_none() {
                        job.chain_result(Resource::new_no_data());
                    } else {
                        job.chain_result(Resource::new_with_data());
                    }
                } else {
                    self.storage.clear();
                    job.chain_result(Resource::new_with_data());
                }
                job.finish();
            }
            StorageJobKind::Search(place, regex_str) => {
                let mut return_res = Resource::new_mutli();
                for (id, res) in self.storage.iter_mut() {
                    if id.place != *place {
                        continue;
                    }
                    if regex_str.is_none() {
                        return_res.chain(res.clone());
                    } else {
                        if id.path.is_some() {
                            let place = id.path.clone();
                            let place = place.unwrap();
                            let place = place.to_str().unwrap();
                            let regex_str = regex_str.as_ref().unwrap();
                            let regex = regex::Regex::new(regex_str)?;
                            if regex.is_match(place) {
                                return_res.chain(res.clone());
                            }
                        }
                    }
                }
                if return_res.has_data() {
                    job.chain_result(return_res);
                } else {
                    job.chain_result(Resource::new_no_data());
                }
                job.finish();
            }
        };
        Ok(None)
    }
}

pub struct LocalStorageLabor {}

pub struct DufsStorageLabor {}
