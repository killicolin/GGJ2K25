#![allow(clippy::type_complexity)]
use bevy::{asset::Assets, math::vec2, prelude::*, transform, window::PresentMode};
use bevy_kira_audio::prelude::*;
use main_menu::main_menu_plugin::MainMenuPlugin;

use avian2d::prelude::*;
mod constants;
mod main_menu;

use constants::*;
use rand::Rng;

#[derive(Component)]
struct Player(u32);

#[derive(Component)]
struct Volume(f32);

#[derive(States, Debug, Clone, PartialEq, Default, Eq, Hash)]
enum MyAppState {
    #[default]
    MainMenu,
    InGame,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let width = 1.0 * 128.;
    let height = 0.2 * 128.;
    commands.spawn((
        RigidBody::Dynamic,
        Collider::rectangle(width, height),
        Mesh2d(meshes.add(Rectangle::new(width, height))),
        MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::css::ORANGE))),
        Transform::default(),
        ColliderDensity(1.5),
        Player(0),
        Volume(width * height),
        ExternalForce::default().with_persistence(false),
    ));

    // Arena
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(ARENA_WIDTH, ARENA_HEIGHT))),
        MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::css::DARK_GREEN))),
        Transform::default().with_translation(Vec3::new(0., 0., -1.)),
    ));
}

//, mut interaction_query: Query<(&Transform), (With<Player>)>

fn spawn_bubble(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Vec3,
    direction: Vec3,
    initial_speed: f32,
    is_colliding: bool,
) {
    if is_colliding {
        commands.spawn((
            RigidBody::Dynamic,
            Collider::circle(BUBBLE_RADIUS),
            Mesh2d(meshes.add(Circle::new(BUBBLE_RADIUS))),
            Volume(BUBBLE_RADIUS * BUBBLE_RADIUS * 2. * std::f32::consts::PI),
            MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::css::BLUE))),
            Transform::from_translation(transform),
            ColliderDensity(0.05),
            LinearVelocity(direction.xy() * initial_speed),
            ExternalForce::default().with_persistence(false),
        ));
    } else {
        commands.spawn((
            RigidBody::Dynamic,
            Mesh2d(meshes.add(Circle::new(BUBBLE_RADIUS))),
            Volume(BUBBLE_RADIUS * BUBBLE_RADIUS * 2. * std::f32::consts::PI),
            MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::css::BLUE))),
            Transform::from_translation(transform),
            Mass(BUBBLE_RADIUS * BUBBLE_RADIUS * 2. * std::f32::consts::PI * 0.05),
            LinearVelocity(direction.xy() * initial_speed),
            ExternalForce::default().with_persistence(false),
        ));
    }
}

fn drag_force(mut in_water_object: Query<(&Volume, &LinearVelocity, &mut ExternalForce)>) {
    for (volume, linear_velocity, mut force) in &mut in_water_object {
        let archimede = FLUID_DENSITY * GRAVITY * volume.0 * Vec2::Y;
        let drag = -DRAG_COEFFICIENT * linear_velocity.0;
        force.apply_force(archimede + drag);
    }
}

fn use_turbo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cachet_query: Query<(&Transform, &mut ExternalForce), (With<Player>)>,
) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    rng.gen_range(-60. ..4.);
    let amplitude = Vec3::Y * TURBO_FORCE;
    let left = Vec3::new(-32., -13., 0.);
    let right = Vec3::new(32., -13., 0.);
    let center = Vec3::new(0., 0., 0.);
    for (transform, mut force) in &mut cachet_query {
        if keyboard_input.pressed(KeyCode::KeyD) {
            force.apply_force_at_point(
                (transform.rotation * amplitude).xy(),
                (transform.rotation * left).xy(),
                (transform.rotation * center).xy(),
            );
            let is_colliding = rng.gen_bool(0.7);
            let pos = if is_colliding { 0. } else { 1. };
            for _ in 1..NB_TURBO_PARTICLE {
                spawn_bubble(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    transform.translation
                        + transform.rotation * Vec3::new(rng.gen_range(-60. ..4.), -13., pos),
                    (transform.rotation * Vec3::NEG_Y),
                    BUBBLE_EMMISSION_SPEED * (1. - pos),
                    is_colliding,
                );
            }
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            force.apply_force_at_point(
                (transform.rotation * amplitude).xy(),
                (transform.rotation * right).xy(),
                (transform.rotation * center).xy(),
            );
            let is_colliding = rng.gen_bool(0.7);
            let pos = if is_colliding { 0. } else { 1. };

            for _ in 1..NB_TURBO_PARTICLE {
                spawn_bubble(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    transform.translation
                        + transform.rotation * Vec3::new(rng.gen_range(4. ..60.), -13., pos),
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
            transform.translation
                + transform.rotation
                    * Vec3::new(rng.gen_range(-60. ..60.), rng.gen_range(-12. ..12.), 1.),
            (transform.rotation * Vec3::NEG_Y),
            0.,
            false,
        );
    }
}

fn update_camera(
    mut camera_query : Query<(&mut Transform, &mut OrthographicProjection), (With<Camera2d>, Without<Player>)>,
    player_query : Query<&Transform, (With<Player>, Without<Camera2d>)>
)
{
    // arena center participation
    let mut interest_area = Rect::new(
        -0.5 * ARENA_WIDTH, -0.5 * ARENA_HEIGHT, 0.5 * ARENA_WIDTH, 0.5 * ARENA_HEIGHT);

    // players participation
    for player_transform in player_query.iter()
    {
        interest_area = interest_area.union_point(player_transform.translation.xy());
    }
    let center = interest_area.center();
    let target_position = Vec3::new(center.x, center.y, 0.);

    for (mut camera_transform, mut cam) in &mut camera_query
    {
        let new_camera_translate = CAM_ELASTICITY * camera_transform.translation + (1.0 - CAM_ELASTICITY) * target_position;
        camera_transform.translation = new_camera_translate;

        let mut cam_area = cam.area;
        cam_area.min += new_camera_translate.xy();
        cam_area.max += new_camera_translate.xy();

        info!("cam_area {:?}", cam_area);

        let mut zoom: f32 = cam.scale;

        if cam_area.union(interest_area) != cam_area
        {
            zoom *= 1.01;
        }

        let inner = cam_area.inflate(-200.);
        if inner.union(interest_area) == inner
        {
            zoom *= 0.99;
        }

        zoom = zoom.clamp(CAM_ZOOM_MIN, CAM_ZOOM_MAX);
        cam.scale = zoom;
    }

}

// fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
//     audio
//         .play(asset_server.load(""))
//         .looped();
// }

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

    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(MainMenuPlugin);
    app.add_plugins(AudioPlugin);

    cfg_if::cfg_if! {
        if #[cfg(not(target_arch = "wasm32"))] {
            app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
        }
    }
    app.add_systems(Startup, setup);

    app.add_systems(OnEnter(MyAppState::InGame), setup_game);
    app.add_systems(Update, update_camera.run_if(in_state(MyAppState::InGame)));

    app.add_systems(
        FixedUpdate,
        (use_turbo, drag_force).run_if(in_state(MyAppState::InGame)),
    );
    // app.add_systems(OnEnter(MyAppState::InGame), start_background_audio);

    app.run();
}
