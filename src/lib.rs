#![allow(clippy::type_complexity)]
use bevy::{asset::Assets, math::vec2, prelude::*, transform, window::PresentMode};
use bevy_kira_audio::prelude::*;
use main_menu::main_menu_plugin::MainMenuPlugin;

use avian2d::prelude::*;
mod constants;
mod main_menu;

use constants::BUBBLE_RADIUS;

#[derive(Component)]
struct Player(u32);

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
        ColliderDensity(0.01),
        Player(0),
        ExternalForce::default().with_persistence(false),
    ));
}

//, mut interaction_query: Query<(&Transform), (With<Player>)>

fn spawn_bubble(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Vec3,
) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::circle(BUBBLE_RADIUS),
        Mesh2d(meshes.add(Circle::new(BUBBLE_RADIUS))),
        MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::css::BLUE))),
        Transform::from_translation(transform),
        ColliderDensity(0.0001),
    ));
}

fn apply_force(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cachet_query: Query<(&Transform, &mut ExternalForce), (With<Player>)>,
) {
    let amplitude = Vec3::Y * 1000.;
    let left = Vec3::new(-32., -13., 0.);
    let right = Vec3::new(32., -13., 0.);
    let center = Vec3::new(0., 0., 0.);
    for (transform, mut force) in &mut cachet_query {
        if keyboard_input.pressed(KeyCode::KeyA) {
            force.apply_force_at_point(
                (transform.rotation * amplitude).xy(),
                (transform.rotation * left).xy(),
                (transform.rotation * center).xy(),
            );
            spawn_bubble(
                &mut commands,
                &mut meshes,
                &mut materials,
                (transform.translation + transform.rotation * left),
            );
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            force.apply_force_at_point(
                (transform.rotation * amplitude).xy(),
                (transform.rotation * right).xy(),
                (transform.rotation * center).xy(),
            );
            spawn_bubble(
                &mut commands,
                &mut meshes,
                &mut materials,
                (transform.translation + transform.rotation * right),
            );
        }
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

    app.add_systems(
        FixedUpdate,
        apply_force.run_if(in_state(MyAppState::InGame)),
    );
    // app.add_systems(OnEnter(MyAppState::InGame), start_background_audio);

    app.run();
}
