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
    Message(Message),
    Matters(Matters),
    Mutli(Vec<Resource>),
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
    pub fn nodata() -> Self {
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
