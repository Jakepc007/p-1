use bevy::prelude::{EventReader, ResMut};

use super::{
    comp::CombatStage,
    event::StartCombat,
    res::{CombatStages, StageRegion, StageType},
};

pub fn start_combat_event_listener(mut reader: EventReader<StartCombat>) {
    for event in reader.iter() {}
}

pub fn DEV_load_combat_stages(mut stages: ResMut<CombatStages>) {
    stages
        .all
        .push(CombatStage::new(StageType::Normal, StageRegion::Normal));
}
