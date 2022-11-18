use bevy::prelude::Component;

#[derive(Component)]
pub struct Health {
    pub value: i8,
    original_value: i8,
}

impl Health {
    pub fn get_size_multiplier(&self) -> f32 {
        self.value as f32 / self.original_value as f32
    }

    pub fn new(value: i8) -> Self {
        Self {
            value,
            original_value: value,
        }
    }
}

#[test]
fn test_health_multiplier() {
    let health = Health::new(5);

    assert_eq!(health.value, health.original_value)
}
