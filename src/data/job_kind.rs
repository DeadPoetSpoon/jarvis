use super::{Resource, ResourceId};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub enum JobKind {
    #[default]
    None,
    Inner(InnerJobKind),
}

impl JobKind  {
    pub fn add_inner_msg(resource:Resource) -> Self {
        JobKind::Inner(InnerJobKind::AddMsg(resource))
    }
    pub fn get_inner_msg(id:Option<ResourceId>) -> Self {
        JobKind::Inner(InnerJobKind::GetMsg(id))
    }
    pub fn remove_inner_msg(id:Option<ResourceId>) -> Self{
        JobKind::Inner(InnerJobKind::RemoveMsg(id))
    }
    pub fn has_inner_msg()->Self {
        JobKind::Inner(InnerJobKind::HasMsg)
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

