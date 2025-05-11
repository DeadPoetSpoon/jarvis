use crate::{Resource, ResourceId};

use super::Labor;
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub enum ScheduleJobKind {
    #[default]
    Nothing,
    SaveMatters(Resource),
    ReadMatters(Option<ResourceId>),
    RemoveMatters(Option<ResourceId>),
}
pub struct ScheduleLabor {
    storage_labor: Box<dyn Labor>,
}

impl ScheduleLabor {}

impl Labor for ScheduleLabor {
    fn handle(&mut self, job: &mut crate::Job) -> anyhow::Result<Option<Vec<crate::Job>>> {
        todo!()
    }
}
