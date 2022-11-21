use crate::damage::comp::DamageSource;

pub struct DeathEvent {
    pub source: DamageSource,
    pub overkill: i8,
}
