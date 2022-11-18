use super::res::{StageRegion, StageType};

pub struct CombatStage {
    stage_type: StageType,
    region: StageRegion,
}

impl CombatStage {
    pub fn new(stage_type: StageType, region: StageRegion) -> Self {
        Self { stage_type, region }
    }
}
