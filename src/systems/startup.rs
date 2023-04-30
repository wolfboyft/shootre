use crate::components::*;
use crate::util::*;
use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_camera (mut commands: Commands) {
    commands.spawn(
        Camera2dBundle {
            ..default()
        }
    );
}

pub fn spawn_player(
    mut commands: Commands
) {
    let position = Vec2::ZERO;
    let angle = 0.0;
    commands.spawn((
        ( // Nested to get around bundle size limit
            Position {value: position},
            PreviousPosition {value: position},
            Velocity {value: Vec2::ZERO},
            Gait {
                standing_max_speed: 200.0,
                standing_acceleration: 800.0,
                floored_max_speed: 100.0,
                floored_acceleration: 400.0,
                floored_recovery_time: 2.0
            },
            FlyingRecoveryRate {value: 800.0},
            RegroundThreshold {value: 210.0},
            TripThreshold {value: 220.0}
        ),
        (
            Angle {value: angle},
            PreviousAngle {value: angle},
            AngularVelocity {value: 0.0},
            AngularGait {
                max_speed: TAU / 2.0,
                acceleration: TAU * 8.0
            },
        ),
        (
            Collider {
                radius: 10.0,
                solid: true
            },
            Mass {value: 100.0},
            Restitution {value: 0.2},
            FloorFriction {value: 300.0}
        ),
        (
            ShapeBundle {
                // Path is created by rebuild_collider_shape before rendering
                ..default()
            },
            Fill::color(Color::WHITE),
            Stroke::new(Color::WHITE, 1.0)
        ),
        Player,
        Will {..default()},
        Grounded {
            standing: true,
            floored_recovery_timer: None
        },
        Holder {pick_up_range: 20.0}
    ));
}

pub fn spawn_other(
    mut commands: Commands
) {
    let _machine_gun = Gun {
        projectile_speed: 2000.0,
        projectile_flying_recovery_rate: 250.0,
        projectile_spread: Vec2::new(0.005, 0.005),
        projectile_count: 1,
        projectile_colour: Color::CYAN,
        muzzle_distance: 5.0,
        cooldown: 0.01,
        auto: true,

        cooldown_timer: 0.0,
        trigger_depressed: false,
        trigger_depressed_previous_frame: false
    };
    let shotgun = Gun {
        projectile_speed: 1750.0,
        projectile_flying_recovery_rate: 500.0,
        projectile_spread: Vec2::new(0.05, 0.05),
        projectile_count: 25,
        projectile_colour: Color::CYAN,
        muzzle_distance: 5.0,
        cooldown: 1.0,
        auto: false,

        cooldown_timer: 0.0,
        trigger_depressed: false,
        trigger_depressed_previous_frame: false
    };
    let position = Vec2::new(100.0, 0.0);
    commands.spawn((
        (
            Position {value: position},
            Velocity {value: Vec2::ZERO}
        ),
        (
            Collider {
                radius: 5.0,
                solid: false
            },
            Mass {value: 10.0},
            Restitution {value: 0.4},
            FloorFriction {value: 200.0}
        ),
        (
            ShapeBundle {
                ..default()
            },
            Fill::color(Color::WHITE),
            Stroke::new(Color::WHITE, 1.0)
        ),
        Grounded {
            standing: false,
            floored_recovery_timer: None
        },
        shotgun,
        Holdable
    ));
}

pub fn spawn_dots(
    mut commands: Commands
) {
    let shape = shapes::Circle {
        radius: 2.0,
        ..default()
    };
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        commands.spawn((
            Position {value: random_in_shape::circle(&mut rng, 1000.0)},
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(Color::NONE),
            Stroke::new(Color::WHITE, 1.0)
        ));
    }
}
