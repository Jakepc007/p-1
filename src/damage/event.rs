use super::comp::{DamageSource, DamageType};

pub struct TakeDamageEvent {
    pub quantity: i8,
    pub damage_type: DamageType,
    pub source: DamageSource,
}
