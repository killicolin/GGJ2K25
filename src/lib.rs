#![allow(clippy::type_complexity)]
use bevy::{
    asset::Assets, prelude::*, reflect::GetTupleField, sprite::Material2dPlugin,
    window::PresentMode,
};
use bevy_kira_audio::prelude::*;
use cachet_material::CachetMaterial;
use game_hud::game_hud_plugin::GameHudPlugin;
use main_menu::main_menu_plugin::MainMenuPlugin;

use avian2d::prelude::*;
mod cachet_material;
mod constants;
mod game_hud;
mod main_menu;
mod my_audio;
mod on_hit;

use constants::*;
use my_audio::my_audio_plugin::MyAudioPlugin;
use on_hit::on_hit_plugin::OnHitPlugin;
use rand::Rng;

pub struct PlayerKeyMap {
    up: KeyCode,
    left: KeyCode,
    right: KeyCode,
    down: KeyCode,
}

#[derive(Resource)]
pub struct PlayerNumber(usize);

#[derive(Component)]
struct InGame;

#[derive(Component)]
struct Player(usize);

#[derive(Component, Copy, Clone)]
struct HudPlayer(usize);

#[derive(Component, Copy, Clone)]
struct HudInnerBar;

#[derive(Component, Debug)]
struct Health(f32);

#[derive(Component)]
struct Bubble;

#[derive(Component)]
struct Glass;

#[derive(Component)]
struct Volume(f32);

#[derive(States, Debug, Clone, PartialEq, Default, Eq, Hash)]
enum MyAppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(States, Debug, Clone, PartialEq, Default, Eq, Hash)]
enum MyMainMenuState {
    #[default]
    MainMenu,
    Help,
    PlayerMenu,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn resetup(mut commands: Commands, query: Query<Entity, With<Camera2d>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn(Camera2d);
}

fn setup_glasses(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut water_color = Color::from(bevy::color::palettes::css::LIGHT_YELLOW);
    water_color.set_alpha(0.1);
    let mut glass_color = Color::from(bevy::color::palettes::css::LIGHT_BLUE);
    glass_color.set_alpha(0.3);

    // Background
    commands.spawn((
        InGame,
        Transform::default()
            .with_translation(Vec3::new(0., 1000., -2.))
            .with_scale(Vec3::new(2., 2., 1.)),
        Sprite::from_image(asset_server.load("sprite/Kitchen.png")),
    ));

    // Arena
    commands.spawn((
        InGame,
        Mesh2d(meshes.add(Rectangle::new(GLASS_RADIUS * 2., WATER_LEVEL))),
        MeshMaterial2d(materials.add(water_color)),
        Transform::default().with_translation(Vec3::new(0., (WATER_LEVEL - GLASS_HEIGHT) / 2., 2.)),
    ));
    commands.spawn((
        InGame,
        Mesh2d(meshes.add(Rectangle::new(GLASS_RADIUS * 2., WATER_LEVEL))),
        MeshMaterial2d(materials.add(water_color)),
        Transform::default().with_translation(Vec3::new(
            0.,
            (WATER_LEVEL - GLASS_HEIGHT) / 2.,
            -1.,
        )),
    ));

    // Glasses BOTTOM
    commands.spawn((
        InGame,
        RigidBody::Static,
        Collider::rectangle(GLASS_RADIUS * 2., GLASS_WIDTH),
        Mesh2d(meshes.add(Rectangle::new(GLASS_RADIUS * 2., GLASS_WIDTH))),
        MeshMaterial2d(materials.add(glass_color)),
        Transform::default().with_translation(Vec3::new(0., -GLASS_HEIGHT / 2., 0.)),
        Glass,
    ));

    // Glasses LEFT
    commands.spawn((
        InGame,
        RigidBody::Static,
        Collider::rectangle(GLASS_WIDTH, GLASS_HEIGHT),
        Mesh2d(meshes.add(Rectangle::new(GLASS_WIDTH, GLASS_HEIGHT))),
        MeshMaterial2d(materials.add(glass_color)),
        Transform::default().with_translation(Vec3::new(-GLASS_RADIUS, 0., 0.)),
        Glass,
    ));

    // Glasses RIGHT
    commands.spawn((
        InGame,
        RigidBody::Static,
        Collider::rectangle(GLASS_WIDTH, GLASS_HEIGHT),
        Mesh2d(meshes.add(Rectangle::new(GLASS_WIDTH, GLASS_HEIGHT))),
        MeshMaterial2d(materials.add(glass_color)),
        Transform::default().with_translation(Vec3::new(GLASS_RADIUS, 0., 0.)),
        Glass,
    ));
}

fn setup_game_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_number: Res<PlayerNumber>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CachetMaterial>>,
) {
    let width = 1.0 * 128.;
    let height = 0.2 * 128.;
    let img = asset_server.load("sprite/Cachet.png");
    for i in 0..player_number.0 {
        commands.spawn((
            InGame,
            RigidBody::Dynamic,
            Collider::rectangle(width, height),
            Mesh2d(meshes.add(Rectangle::new(width, height))),
            MeshMaterial2d(materials.add(CachetMaterial {
                color: Color::from(PLAYER_COLOR[i]).to_linear(),
                color_texture: Some(img.clone()),
            })),
            Transform::default().with_translation(PLAYER_POSITION[player_number.0 - 1][i]),
            ColliderDensity(CACHET_DENSITY),
            Player(i),
            Health(INITIAL_HEALTH),
            Volume(width * height),
            ExternalForce::default().with_persistence(false),
        ));
    }
}

fn spawn_bubble(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player: usize,
    transform: Vec3,
    direction: Vec3,
    initial_speed: f32,
    is_colliding: bool,
) {
    let bubble_color =
        Color::from(PLAYER_COLOR[player]).mix(&Color::from(bevy::color::palettes::css::WHITE), 0.5);
    if is_colliding {
        commands.spawn((
            InGame,
            Bubble,
            RigidBody::Dynamic,
            Collider::circle(BUBBLE_RADIUS),
            Mesh2d(meshes.add(Circle::new(BUBBLE_RADIUS))),
            Volume(BUBBLE_RADIUS * BUBBLE_RADIUS * 2. * std::f32::consts::PI),
            MeshMaterial2d(materials.add(bubble_color)),
            Transform::from_translation(transform),
            ColliderDensity(0.1),
            LinearVelocity(direction.xy() * initial_speed),
            ExternalForce::default().with_persistence(false),
        ));
    } else {
        commands.spawn((
            InGame,
            Bubble,
            RigidBody::Dynamic,
            Mesh2d(meshes.add(Circle::new(BUBBLE_RADIUS))),
            Volume(BUBBLE_RADIUS * BUBBLE_RADIUS * 2. * std::f32::consts::PI),
            MeshMaterial2d(materials.add(bubble_color)),
            Transform::from_translation(transform),
            Mass(BUBBLE_RADIUS * BUBBLE_RADIUS * 2. * std::f32::consts::PI * 0.1),
            LinearVelocity(direction.xy() * initial_speed),
            ExternalForce::default().with_persistence(false),
        ));
    }
}

fn bubble_emiter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cachet_query: Query<(&Transform, &Player, &Health), With<Player>>,
) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    rng.gen_range(-60. ..4.);
    for (transform, player, health) in &mut cachet_query {
        if is_in_water(&transform.translation) && health.0 > 0. {
            if keyboard_input.pressed(PLAYER_CONTROL[player.0].right)
                || keyboard_input.pressed(PLAYER_CONTROL[player.0].up)
            {
                let is_colliding = rng.gen_bool(0.3);
                let pos = if is_colliding { 0. } else { 1. };
                for _ in 1..NB_TURBO_PARTICLE {
                    spawn_bubble(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        player.0,
                        transform.translation
                            + transform.rotation * Vec3::new(rng.gen_range(-60. ..4.), -13., pos),
                        (transform.rotation * Vec3::NEG_Y),
                        BUBBLE_EMMISSION_SPEED * (1. - pos),
                        is_colliding,
                    );
                }
            }
            if keyboard_input.pressed(PLAYER_CONTROL[player.0].left)
                || keyboard_input.pressed(PLAYER_CONTROL[player.0].up)
            {
                let is_colliding = rng.gen_bool(0.3);
                let pos = if is_colliding { 0. } else { 1. };
                for _ in 1..NB_TURBO_PARTICLE {
                    spawn_bubble(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        player.0,
                        transform.translation
                            + transform.rotation * Vec3::new(rng.gen_range(4. ..60.), -13., pos),
                        (transform.rotation * Vec3::NEG_Y),
                        BUBBLE_EMMISSION_SPEED * (1. - pos),
                        is_colliding,
                    );
                }
            }
            if keyboard_input.pressed(PLAYER_CONTROL[player.0].down) {
                let is_colliding = rng.gen_bool(0.7);
                let pos = if is_colliding { 0. } else { 1. };
                for _ in 1..NB_TURBO_PARTICLE * 2 {
                    spawn_bubble(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        player.0,
                        transform.translation
                            + transform.rotation * Vec3::new(rng.gen_range(-60. ..60.), 13., pos),
                        (transform.rotation * Vec3::NEG_Y),
                        BUBBLE_EMMISSION_SPEED * (1. - pos),
                        is_colliding,
                    );
                }
            }
            spawn_bubble(
                &mut commands,
                &mut meshes,
                &mut materials,
                player.0,
                transform.translation
                    + transform.rotation
                        * Vec3::new(rng.gen_range(-60. ..60.), rng.gen_range(-12. ..12.), 1.),
                (transform.rotation * Vec3::NEG_Y),
                0.,
                false,
            );
        }
    }
}

fn drag_force(
    mut in_water_object: Query<(
        &Transform,
        &Volume,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut ExternalForce,
        &ComputedMass,
    )>,
) {
    for (transform, volume, mut linear_velocity, mut angular_velocity, mut force, mass) in
        &mut in_water_object
    {
        if is_in_water(&transform.translation) {
            let archimede = FLUID_DENSITY * GRAVITY * volume.0 * Vec2::Y;
            linear_velocity.0 = (1. - DRAG_WATER_COEFFICIENT) * linear_velocity.0;
            angular_velocity.0 = (1. - DRAG_WATER_COEFFICIENT) * angular_velocity.0;
            force.apply_force(archimede);
        } else {
            let double_gravity = GRAVITY * GRAVITY_SCALE * 3.0 * mass.value() * Vec2::NEG_Y;
            linear_velocity.0 = (1. - DRAG_AIR_COEFFICIENT) * linear_velocity.0;
            angular_velocity.0 = (1. - DRAG_AIR_COEFFICIENT) * angular_velocity.0;
            force.apply_force(double_gravity);
        }
    }
}

pub fn is_in_water(translation: &Vec3) -> bool {
    translation.y <= (WATER_LEVEL * 0.5 - (GLASS_HEIGHT - WATER_LEVEL) / 2.) - 10.
        && translation.y >= GLASS_HEIGHT * -0.5
        && translation.x >= -GLASS_RADIUS
        && translation.x <= GLASS_RADIUS
}

fn use_turbo(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cachet_query: Query<(&Transform, &Player, &mut ExternalForce, &mut Health), With<Player>>,
) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    rng.gen_range(-60. ..4.);
    let amplitude = Vec3::Y * TURBO_FORCE;
    let left_bottom = Vec3::new(-32., -13., 0.);
    let right_bottom = Vec3::new(32., -13., 0.);
    let top = Vec3::new(0., 13., 0.);
    let center = Vec3::new(0., 0., 0.);
    for (transform, player, mut force, mut health) in &mut cachet_query {
        if is_in_water(&transform.translation) && health.0 > 0.{
            if keyboard_input.pressed(PLAYER_CONTROL[player.0].right)
                || keyboard_input.pressed(PLAYER_CONTROL[player.0].up)
            {
                force.apply_force_at_point(
                    (transform.rotation * amplitude).xy(),
                    (transform.rotation * left_bottom).xy(),
                    (transform.rotation * center).xy(),
                );
                health.0 -= TURBO_TICK_DAMAGE * GLOBAL_DAMAGE_SCALE;
            }
            if keyboard_input.pressed(PLAYER_CONTROL[player.0].left)
                || keyboard_input.pressed(PLAYER_CONTROL[player.0].up)
            {
                force.apply_force_at_point(
                    (transform.rotation * amplitude).xy(),
                    (transform.rotation * right_bottom).xy(),
                    (transform.rotation * center).xy(),
                );
                health.0 -= TURBO_TICK_DAMAGE * GLOBAL_DAMAGE_SCALE;
            }
            if keyboard_input.pressed(PLAYER_CONTROL[player.0].down) {
                force.apply_force_at_point(
                    (transform.rotation * -amplitude).xy(),
                    (transform.rotation * top).xy(),
                    (transform.rotation * center).xy(),
                );
                health.0 -= TURBO_TICK_DAMAGE * GLOBAL_DAMAGE_SCALE;
            }
        }
    }
}

fn update_camera(
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<Camera2d>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    // arena center participation
    let mut interest_area = Rect::new(
        -0.5 * GLASS_RADIUS * 2.0,
        -0.5 * GLASS_HEIGHT,
        0.5 * GLASS_RADIUS * 2.0,
        0.5 * GLASS_HEIGHT,
    );

    // players participation
    for player_transform in player_query.iter() {
        interest_area = interest_area.union_point(player_transform.translation.xy());
    }

    // buffer
    interest_area = interest_area.inflate(interest_area.size().max_element() * CAM_BUFFER);
    let center = interest_area.center();
    let target_position = Vec3::new(center.x, center.y, 0.);

    for (mut camera_transform, mut cam) in &mut camera_query {
        let new_camera_translate = CAM_ELASTICITY * camera_transform.translation
            + (1.0 - CAM_ELASTICITY) * target_position;
        camera_transform.translation = new_camera_translate;

        let mut cam_area = cam.area;
        cam_area.min += new_camera_translate.xy();
        cam_area.max += new_camera_translate.xy();

        let mut zoom: f32 = cam.scale;

        if cam_area.union(interest_area) != cam_area {
            zoom *= 1. + CAM_ZOOM_SPEED;
        }

        let inner = cam_area.inflate(-200.);
        if inner.union(interest_area) == inner {
            zoom *= 1. - CAM_ZOOM_SPEED;
        }

        zoom = zoom.clamp(CAM_ZOOM_MIN, CAM_ZOOM_MAX);
        cam.scale = zoom;
    }
}

fn update_health(mut query: Query<(&mut Health, &Transform)>) {
    for (mut health, transform) in &mut query {
        if is_in_water(&transform.translation) {
            health.0 -= GLOBAL_DAMAGE_SCALE * WATER_TICK_DAMAGE;
        }
    }
}

fn try_kill_bubbles(mut commands: Commands, query: Query<(Entity, &Transform), With<Bubble>>) {
    for (entity, transform) in query.iter() {
        if !is_in_water(&transform.translation) {
            commands.entity(entity).despawn();
        }
    }
}

fn try_kill_by_health(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Health,
        &Player,
        &mut ColliderDensity,
        &mut Transform,
        &mut LinearVelocity
    )>)
{
    for (entity, health, player, mut density, mut transform, mut vel) in query.iter_mut() {
        if health.0 <= 0. {
            if transform.scale.x == 1.
            {
                // bye bye message
                let mut rng = rand::thread_rng();
                let choice = rng.gen_range(0..3);
                match choice {
                    0 => warn!("Player {:?} disolved :'(", player.0),
                    1 => warn!("Player {:?} didn't want to fight anymore", player.0),
                    _ => warn!("Player {:?} left the battle arena", player.0),
                }
            }

            transform.scale -= 0.005;
            vel.0 *= 0.95;
            density.0 = CACHET_DENSITY / 3.;

            if transform.scale.x < 0.05
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn end_game_condition(
    player_number: Res<PlayerNumber>,
    mut app_state: ResMut<NextState<MyAppState>>,
    query: Query<&Health, With<Player>>,
) {
    let mut alive_players = 0;
    for health in query.iter() {
        if health.0 > 0. {
            alive_players += 1;
        }
    }

    if (player_number.0 == 1 && alive_players <= 0) || (player_number.0 != 1 && alive_players <= 1)
    {
        app_state.set(MyAppState::MainMenu);
    }
}

// kill the player when they are out of the playable area
fn try_kill_by_zone(mut query: Query<(&mut Health, &Transform), With<Player>>) {
    for (mut health, transform) in query.iter_mut() {
        if transform.translation.y < GLASS_HEIGHT * -0.5 {
            health.0 = 0.;
            warn!("player left the area like a wuss");
        }
    }
}

fn on_game_exit(mut commands: Commands, query: Query<Entity, With<InGame>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn run() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "GAME_NAME".to_string(),
            canvas: Some("#my-bevy".into()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }));
    app.init_state::<MyAppState>();
    app.insert_resource(Gravity(Vec2::NEG_Y * GRAVITY * GRAVITY_SCALE));

    app.add_plugins(Material2dPlugin::<CachetMaterial>::default());
    app.add_plugins(PhysicsPlugins::default());

    app.add_plugins(MainMenuPlugin);
    app.add_plugins(AudioPlugin);
    app.add_plugins(MyAudioPlugin);
    app.add_plugins(GameHudPlugin);
    app.add_plugins(OnHitPlugin);
    cfg_if::cfg_if! {
        if #[cfg(not(target_arch = "wasm32"))] {
            app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
        }
    }
    app.add_systems(Startup, setup);

    app.add_systems(OnEnter(MyAppState::InGame), resetup);
    app.add_systems(OnEnter(MyAppState::InGame), setup_game_player);
    app.add_systems(OnEnter(MyAppState::InGame), setup_glasses);

    app.add_systems(Update, update_camera.run_if(in_state(MyAppState::InGame)));

    app.add_systems(
        FixedUpdate,
        (use_turbo, bubble_emiter, drag_force, update_health).run_if(in_state(MyAppState::InGame)),
    );

    app.add_systems(
        FixedPostUpdate,
        (
            try_kill_bubbles,
            try_kill_by_health,
            try_kill_by_zone,
            end_game_condition,
        )
            .run_if(in_state(MyAppState::InGame)),
    );
    app.add_systems(OnExit(MyAppState::InGame), on_game_exit);

    app.run();
}
