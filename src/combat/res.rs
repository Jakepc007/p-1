use rand::Rng;

use super::comp::CombatStage;

#[derive(Default)]
pub struct CombatStages {
    pub all: Vec<CombatStage>,
}

pub enum StageRegion {
    Normal,
    Graveyard,
}

pub enum StageType {
    Normal,
}

impl CombatStages {
    pub fn get_random(&self, stage_type: StageType) -> usize {
        let mut rng = rand::thread_rng();
        match stage_type {
            StageType::Normal => {
                let index = rng.gen_range(0..self.all.len());
                index
            }
        }
    }
}
