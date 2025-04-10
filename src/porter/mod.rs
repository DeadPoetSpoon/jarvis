use crate::Mission;

pub trait porter {
    fn handle(mission: Mission) {}
}
