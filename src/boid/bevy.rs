use bevy::math;
use bevy::prelude::{
    App, Color, Commands, Component, Input, KeyCode, Plugin, Query, Res, Transform, Vec2, Vec3,
    With,
};
use bevy::sprite::{Sprite, SpriteBundle};

const BOID_COUNT: i64 = 1;
const BOID_CENTER_FORCE: f32 = 1.5;
const BOID_PUSH_FACTOR: f32 = 0.2;
const BOID_AREA_FACTOR: f32 = 0.8;
const BOID_AREA_INFLUENCE_ON_CENTER: f32 = 0.12;

pub struct BoidPlugin;

#[derive(Component)]
pub struct Boid {
    anchor: Vec2,
    area: f32,
    target_position: Vec2,
}

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(dev_spawn_boids);
        app.add_system(anchor_boids);
        app.add_system(interact_boids);
        app.add_system(dev_spawn_boid_on_space);
    }
}

fn dev_spawn_boids(mut commands: Commands) {
    for _ in 0..BOID_COUNT {
        let rand_x = (rand::random::<f32>() * 400.0) - 200.0;
        let rand_y = (rand::random::<f32>() * 400.0) - 200.0;
        let size = (rand::random::<f32>() * 10.0) + 15.0;

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                transform: Transform {
                    scale: Vec3::new(size, size, 1.0),
                    translation: Vec3::new(rand_x, rand_y, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Boid {
                anchor: Vec2::new(0.0, 0.0),
                area: size,
                target_position: Vec2::new(rand_x, rand_y),
            });
    }
}

// pull boids towards their anchor
fn anchor_boids(mut boids: Query<(&mut Transform, &Boid), With<Boid>>) {
    for (
        mut boid_transform,
        Boid {
            anchor,
            area,
            target_position,
        },
    ) in boids.iter_mut()
    {
        let difference = *anchor - boid_transform.translation.truncate();
        let distance = difference.length();
        let direction = difference.normalize();

        boid_transform.translation += Vec3::new(
            direction.x
                * (distance / area)
                * BOID_CENTER_FORCE
                * (area * BOID_AREA_INFLUENCE_ON_CENTER),
            direction.y
                * (distance / area)
                * BOID_CENTER_FORCE
                * (area * BOID_AREA_INFLUENCE_ON_CENTER),
            0.0,
        );
    }
}

// interact boids with each other
fn interact_boids(mut boids: Query<(&mut Transform, &mut Boid), With<Boid>>) {
    let total_boids = boids.iter().count();
    dbg!(total_boids);
    let mut combinations = boids.iter_combinations_mut();
    while let Some([boid1, boid2]) = combinations.fetch_next() {
        let (mut boid1_transform, boid1_boid) = boid1;
        let (mut boid2_transform, boid2_boid) = boid2;

        let difference =
            boid1_transform.translation.truncate() - boid2_transform.translation.truncate();

        // println!("difference: {:?}", difference);

        let distance = difference.length();
        let direction = difference.normalize();

        // println!("direction: {:?}", direction);

        let var_name = 1.9;
        boid1_transform.translation.x += (direction.x * BOID_PUSH_FACTOR)
            * (boid2_boid.area * BOID_AREA_FACTOR)
            / ((total_boids as f32).sqrt() / var_name);
        boid1_transform.translation.y += (direction.y * BOID_PUSH_FACTOR)
            * (boid2_boid.area * BOID_AREA_FACTOR)
            / ((total_boids as f32).sqrt() / var_name);

        boid2_transform.translation.x -= (direction.x * BOID_PUSH_FACTOR)
            * (boid1_boid.area * BOID_AREA_FACTOR)
            / ((total_boids as f32).sqrt() / var_name);
        boid2_transform.translation.y -= (direction.y * BOID_PUSH_FACTOR)
            * (boid1_boid.area * BOID_AREA_FACTOR)
            / ((total_boids as f32).sqrt() / var_name);
    }
}

fn dev_spawn_boid_on_space(mut commands: Commands, input: Res<Input<KeyCode>>) {
    if !input.pressed(KeyCode::Space) {
        return;
    }

    let rand_x = (rand::random::<f32>() * 400.0) - 200.0;
    let rand_y = (rand::random::<f32>() * 400.0) - 200.0;
    let size = (rand::random::<f32>() * 10.0) + 15.0;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(size, size, 1.0),
                translation: Vec3::new(rand_x, rand_y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Boid {
            anchor: Vec2::new(0.0, 0.0),
            area: size,
            target_position: Vec2::new(rand_x, rand_y),
        });
}
