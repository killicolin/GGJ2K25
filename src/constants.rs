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
