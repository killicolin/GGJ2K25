use bevy::{
    color::{Color, Srgba},
    math::Vec3,
    prelude::KeyCode,
};

use crate::PlayerKeyMap;

pub const GRAVITY: f32 = 9.8;
pub const GRAVITY_SCALE: f32 = 12.;
pub const FLUID_DENSITY: f32 = 6.5; // Densité de l'eau en kg/m^3
pub const DRAG_WATER_COEFFICIENT: f32 = 0.01;
pub const DRAG_AIR_COEFFICIENT: f32 = 0.003;

pub const CACHET_DENSITY: f32 = 10.;

pub const BUBBLE_RADIUS: f32 = 2.;
pub const BUBBLE_EMMISSION_SPEED: f32 = 100.;

pub const TURBO_FORCE: f32 = 7500000. * 4.;
pub const NB_TURBO_PARTICLE: usize = 5;

// Camera
pub const CAM_ELASTICITY: f32 = 0.95;
pub const CAM_ZOOM_SPEED: f32 = 0.01;
pub const CAM_ZOOM_MIN: f32 = 0.1; // zoom in
pub const CAM_ZOOM_MAX: f32 = 20.; // zoom out
pub const CAM_BUFFER: f32 = 0.15; // buffer pct

// Arena
pub const GLASS_RADIUS: f32 = 800.;
pub const GLASS_HEIGHT: f32 = 1600.;
pub const WATER_LEVEL: f32 = 1400.;
pub const GLASS_WIDTH: f32 = 30.;

// Health
pub const INITIAL_HEALTH: f32 = 1000.;
pub const GLOBAL_DAMAGE_SCALE: f32 = 0.1;
pub const WATER_TICK_DAMAGE: f32 = 1.;
pub const TURBO_TICK_DAMAGE: f32 = 3.;

// PLAYER
pub const PLAYER_COLOR: [Srgba; 4] = [
    bevy::color::palettes::css::ORANGE,
    bevy::color::palettes::css::LIGHT_PINK,
    bevy::color::palettes::css::LIGHT_GOLDENROD_YELLOW,
    bevy::color::palettes::css::LAVENDER_BLUSH,
];

pub const PLAYER_POSITION: [[Vec3; 4]; 4] = [
    [
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 0., 0.),
    ],
    [
        Vec3::new(-GLASS_RADIUS / 4.0, 0., 0.),
        Vec3::new(GLASS_RADIUS / 4.0, 0., 0.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 0., 0.),
    ],
    [
        Vec3::new(-GLASS_RADIUS * 3. / 5.0, 0., 0.),
        Vec3::new(0., 0., 0.),
        Vec3::new(GLASS_RADIUS * 3. / 5.0, 0., 0.),
        Vec3::new(0., 0., 0.),
    ],
    [
        Vec3::new(-GLASS_RADIUS * 4. / 6.0, 0., 0.),
        Vec3::new(-GLASS_RADIUS / 6.0, 0., 0.),
        Vec3::new(GLASS_RADIUS / 6.0, 0., 0.),
        Vec3::new(GLASS_RADIUS * 4. / 6.0, 0., 0.),
    ],
];

pub const PLAYER_CONTROL: [PlayerKeyMap; 4] = [
    PlayerKeyMap {
        up: KeyCode::KeyW,
        left: KeyCode::KeyA,
        right: KeyCode::KeyD,
        down: KeyCode::KeyS,
    },
    PlayerKeyMap {
        up: KeyCode::ArrowUp,
        left: KeyCode::ArrowLeft,
        right: KeyCode::ArrowRight,
        down: KeyCode::ArrowDown,
    },
    PlayerKeyMap {
        up: KeyCode::KeyY,
        left: KeyCode::KeyG,
        right: KeyCode::KeyJ,
        down: KeyCode::KeyH,
    },
    PlayerKeyMap {
        up: KeyCode::KeyP,
        left: KeyCode::KeyL,
        right: KeyCode::Period,
        down: KeyCode::Semicolon,
    },
];
