use std::{fmt::Display, path::PathBuf};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ResourceId {
    pub place: String,
    pub path: PathBuf,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub enum ResourceData {
    #[default]
    NoData,
    Sample(String),
    Error(String),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Resource {
    pub id: ResourceId,
    pub data: ResourceData,
}

impl Resource {
    pub fn pkg_error<T>(err: T) -> Self
    where
        T: Display,
    {
        Resource {
            data: ResourceData::Error(format!("Error: {err}")),
            ..Default::default()
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
