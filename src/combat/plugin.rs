use bevy::prelude::Plugin;

use super::{res::CombatStages, sys::DEV_load_combat_stages};

pub struct CombatPlugin;

// impl Plugin for CombatPlugin {
//     fn build(&self, app: &mut bevy::prelude::App) {
//         app.init_resource::<CombatStages>()
//             .add_system(DEV_load_combat_stages);
//     }
// }

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CombatStages { all: vec![] })
            .add_system(DEV_load_combat_stages);
    }
}
