use std::{borrow::BorrowMut, fmt::Display, path::PathBuf};

use uuid::Uuid;

use super::{Matters, Message};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceId {
    pub uid: Uuid,
    pub place: Option<String>,
    pub path: Option<PathBuf>,
}

impl Default for ResourceId {
    fn default() -> Self {
        Self {
            uid: Uuid::new_v4(),
            place: Default::default(),
            path: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub enum ResourceData {
    #[default]
    NoData,
    WithData,
    Message(Message),
    Matters(Matters),
    Mutli(Vec<Resource>),
}

impl ResourceData {
    pub fn gene_path(&self) -> Option<PathBuf> {
        match self {
            ResourceData::Matters(matters) => Some(matters.gene_path()),
            _ => None,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Resource {
    pub id: ResourceId,
    pub data: ResourceData,
}

impl Resource {
    pub fn is_me(&self, id: &Uuid) -> bool {
        self.id.uid == *id
    }
    pub fn has_data(&self) -> bool {
        !self.is_no_data()
    }
    pub fn is_no_data(&self) -> bool {
        match &self.data {
            ResourceData::NoData => true,
            ResourceData::Mutli(vec)=>!vec.iter().any(|_|true),
            _=>false
        }
    }
    pub fn new_with_data() -> Self {
        Resource {
            data: ResourceData::WithData,
            ..Default::default()
        }
    }
    pub fn new_no_data() -> Self {
        Resource {
            data: ResourceData::NoData,
            ..Default::default()
        }
    }
    pub fn pkg_simple_msg<T>(msg: T) -> Self
    where
        T: Display,
    {
        Resource {
            data: ResourceData::Message(Message::pkg_simple(msg)),
            ..Default::default()
        }
    }
    pub fn pkg_error<T>(err: T) -> Self
    where
        T: Display,
    {
        Resource {
            data: ResourceData::Message(Message::pkg_error(err)),
            ..Default::default()
        }
    }
    pub fn new_mutli() -> Self {
        Resource {
            data: ResourceData::Mutli(Vec::new()),
            ..Default::default()
        }
    }
    pub fn chain(&mut self, other: Resource) -> &mut Self {
        if let ResourceData::Mutli(vec) = self.data.borrow_mut() {
            vec.push(other);
        } else {
            let old = self.clone();
            let vec = vec![old, other];
            self.data = ResourceData::Mutli(vec);
        }
        self
    }
    pub fn gene_path(&mut self) {
        if let Some(data_path) = self.data.gene_path() {
            if let Some(o) = &self.id.path {
                let path = o.clone().join(data_path);
                self.id.path = Some(path);
            }   
        }
    }
}

impl From<ehttp::Response> for Resource {
    fn from(value: ehttp::Response) -> Self {
        if value.ok {
            if let Some(text) = value.text() {
                if text.is_empty() {
                    return Resource::default();
                } else {
                    match ron::from_str(text) {
                        Ok(resource) => return resource,
                        Err(err) => return Resource::pkg_error(err),
                    }
                }
            } else {
                return Resource::default();
            }
        }
        Resource::pkg_error(format!(
            "From http response error: {}(status: {})",
            value.status_text, value.status
        ))
    }
}
