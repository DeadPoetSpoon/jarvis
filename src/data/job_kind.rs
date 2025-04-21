use super::{Resource, ResourceId};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub enum JobKind {
    #[default]
    None,
    Inner(InnerJobKind),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub enum InnerJobKind {
    #[default]
    Nothing,
    AddMsg(Resource),
    GetMsg(Option<ResourceId>),
    RemoveMsg(Option<ResourceId>),
}
