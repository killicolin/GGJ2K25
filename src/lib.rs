#![allow(clippy::type_complexity)]
use bevy::{asset::Assets, prelude::*, sprite::Material2dPlugin, window::PresentMode, ui::widget::NodeImageMode};
use bevy_kira_audio::prelude::*;
use cachet_material::CachetMaterial;
use main_menu::main_menu_plugin::MainMenuPlugin;

use avian2d::prelude::*;
mod cachet_material;
mod constants;
mod main_menu;

use constants::*;
use rand::Rng;

#[derive(Component)]
struct InGame;

#[derive(Component)]
struct Player(u32);

#[derive(Component)]
struct HudPlayer(u32);

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CachetMaterial>>,
) {
    let width = 1.0 * 128.;
    let height = 0.2 * 128.;
    let img = asset_server.load("sprite/Cachet.png");
    let material_asset = materials.add(CachetMaterial {
        color: Color::from(bevy::color::palettes::css::ORANGE).to_linear(),
        color_texture: Some(img),
    });
    commands.spawn((
        InGame,
        RigidBody::Dynamic,
        Collider::rectangle(width, height),
        Mesh2d(meshes.add(Rectangle::new(width, height))),
        MeshMaterial2d(material_asset),
        Transform::default(),
        ColliderDensity(CACHET_DENSITY),
        Player(0),
        Health(INITIAL_HEALTH),
        Volume(width * height),
        ExternalForce::default().with_persistence(false),
    ));
}

fn player_hit_player(
    collisions: Res<Collisions>,
    mut query: Query<(&LinearVelocity, Entity, &mut Health), (With<Player>)>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([c1, c2]) = combinations.fetch_next() {
        let (velocity1, e1, mut h1) = c1;
        let (velocity2, e2, mut h2) = c2;
        if let Some(player_clash) = collisions.get(e1, e2) {
            let v1 = velocity1.0.distance(Vec2::default());
            let v2 = velocity2.0.distance(Vec2::default());
            let total = v1 + v2;
            let ratio1 = v1 / total;
            let ratio1 = v2 / total;
            println!("{}", total);
            h1.0 -= f32::min(v2 / 10., 20.);
            h2.0 -= f32::min(v1 / 10., 20.);
            // Play Sound
        }
    }
}

fn player_hit_wall(
    asset_server: Res<AssetServer>,
    audio: Res<AudioChannel<GlassChannel>>,
    collisions: Res<Collisions>,
    mut query_player: Query<(Entity, &LinearVelocity, &mut Health), (With<Player>, Without<Glass>)>,
    query_glass: Query<Entity, (With<Glass>, Without<Player>)>,
) {
    for (entity_player, player_velocity, mut heath) in &mut query_player {
        for (entity_wall) in &query_glass {
            if let Some(player_clash) = collisions.get(entity_player, entity_wall) {
                let v = player_velocity.0.distance(Vec2::default());
                let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                if v > 60. && !audio.is_playing_sound() {
                    heath.0 -= f32::min((v / 20.), 20.);
                    audio.play(
                        asset_server
                            .load(format!("audio/Sfx_impactglass{}.wav", rng.gen_range(1..=2))),
                    );
                }
            }
        }
    }
}

//, mut interaction_query: Query<(&Transform), (With<Player>)>

fn spawn_bubble(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Vec3,
    direction: Vec3,
    initial_speed: f32,
    is_colliding: bool,
) {
    let bubble_color = Color::from(bevy::color::palettes::css::ORANGE)
        .mix(&Color::from(bevy::color::palettes::css::WHITE), 0.5);
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

fn is_in_water(translation: &Vec3) -> bool {
    translation.y <= (WATER_LEVEL * 0.5 - (GLASS_HEIGHT - WATER_LEVEL) / 2.) - 10.
        && translation.y >= GLASS_HEIGHT * -0.5
        && translation.x >= -GLASS_RADIUS
        && translation.x <= GLASS_RADIUS
}

fn play_turbo_sound1(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio1: Res<AudioChannel<TurboChannel1p1>>,
    audio2: Res<AudioChannel<TurboChannel2p1>>,
) {
    if keyboard_input.pressed(KeyCode::KeyD)
        || keyboard_input.pressed(KeyCode::KeyW)
        || keyboard_input.pressed(KeyCode::KeyS)
    {
        if !audio1.is_playing_sound() {
            audio1
                .play(asset_server.load("audio/Sfx_boost1.wav"))
                .loop_from(1.0);
        }
    } else if audio1.is_playing_sound() {
        audio1.stop();
    }

    if keyboard_input.pressed(KeyCode::KeyA)
        || keyboard_input.pressed(KeyCode::KeyW)
        || keyboard_input.pressed(KeyCode::KeyS)
    {
        if !audio2.is_playing_sound() {
            audio2
                .play(asset_server.load("audio/Sfx_boost2.wav"))
                .loop_from(1.0);
        }
    } else if audio2.is_playing_sound() {
        audio2.stop();
    }
}

fn play_turbo_sound2(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio1: Res<AudioChannel<TurboChannel1p2>>,
    audio2: Res<AudioChannel<TurboChannel2p2>>,
) {
}

fn play_turbo_sound3(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio1: Res<AudioChannel<TurboChannel1p2>>,
    audio2: Res<AudioChannel<TurboChannel2p2>>,
) {
}

fn play_turbo_sound4(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio1: Res<AudioChannel<TurboChannel1p2>>,
    audio2: Res<AudioChannel<TurboChannel2p2>>,
) {
}

fn use_turbo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cachet_query: Query<(&Transform, &mut ExternalForce, &mut Health), With<Player>>,
) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    rng.gen_range(-60. ..4.);
    let amplitude = Vec3::Y * TURBO_FORCE;
    let left_bottom = Vec3::new(-32., -13., 0.);
    let right_bottom = Vec3::new(32., -13., 0.);
    let top = Vec3::new(0., 13., 0.);
    let center = Vec3::new(0., 0., 0.);
    for (transform, mut force, mut health) in &mut cachet_query {
        if is_in_water(&transform.translation) {
            if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::KeyW) {
                force.apply_force_at_point(
                    (transform.rotation * amplitude).xy(),
                    (transform.rotation * left_bottom).xy(),
                    (transform.rotation * center).xy(),
                );
                health.0 -= TURBO_TICK_DAMAGE * GLOBAL_DAMAGE_SCALE;
                let is_colliding = rng.gen_bool(0.3);
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
            if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::KeyW) {
                force.apply_force_at_point(
                    (transform.rotation * amplitude).xy(),
                    (transform.rotation * right_bottom).xy(),
                    (transform.rotation * center).xy(),
                );
                health.0 -= TURBO_TICK_DAMAGE * GLOBAL_DAMAGE_SCALE;
                let is_colliding = rng.gen_bool(0.3);
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
            if keyboard_input.pressed(KeyCode::KeyS) {
                force.apply_force_at_point(
                    (transform.rotation * -amplitude).xy(),
                    (transform.rotation * top).xy(),
                    (transform.rotation * center).xy(),
                );
                health.0 -= TURBO_TICK_DAMAGE * GLOBAL_DAMAGE_SCALE;
                let is_colliding = rng.gen_bool(0.7);
                let pos = if is_colliding { 0. } else { 1. };

                for _ in 1..NB_TURBO_PARTICLE * 2 {
                    spawn_bubble(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
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

// fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
//     audio
//         .play(asset_server.load(""))
//         .looped();
// }

fn update_health(mut query: Query<(&mut Health, &Transform)>) {
    for (mut health, transform) in &mut query {
        if is_in_water(&transform.translation) {
            health.0 -= GLOBAL_DAMAGE_SCALE * WATER_TICK_DAMAGE;
        }
    }
}

fn update_ui(
    mut query_players: Query<(&Health, &Player)>,
    mut query_ui: Query<(&mut Node, &HudPlayer)>
) {
    for (health, player) in &mut query_players
    {
        for (mut node, hudplayer) in &mut query_ui
        {
            if hudplayer.0 == player.0
            {
                let min = 13.;
                node.width = Val::Percent(min + (100. - min) * health.0 / INITIAL_HEALTH);
            }
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
    mut app_state: ResMut<NextState<MyAppState>>,
    query: Query<&Health, With<Player>>,
) {
    for health in query.iter() {
        if health.0 < 0. {
            warn!("health depleted: {:?}", health);
            app_state.set(MyAppState::MainMenu);
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {

    let image_outer_bar = asset_server.load("sprite/bar_outer.png");
    let image_inner_bar = asset_server.load("sprite/bar_inner.png");

    let slicer = TextureSlicer {
        border: BorderRect::square(64.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };
    commands
        .spawn((InGame, Node {
            width: Val::Percent(100.0),
            height: Val::Percent(12.0),
            top: Val::Percent(86.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        }))
        .with_children(|parent| {
            for (w, h, tag) in [
                (100.0, 32.0, HudPlayer(0)),
                (100.0, 32.0, HudPlayer(1)),
                (100.0, 32.0, HudPlayer(2)),
                (100.0, 32.0, HudPlayer(3))
                ] {
                parent
                    .spawn((
                        InGame,
                        Button,
                        ImageNode {
                            image: image_outer_bar.clone(),
                            image_mode: NodeImageMode::Sliced(slicer.clone()),
                            ..default()
                        },
                        Node {
                            width: Val::Vw(w),
                            height: Val::Px(h),
                            // horizontally center child text
                            justify_content: JustifyContent::Start,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ))
                    .with_child((
                        InGame,
                        tag,
                        ImageNode {
                            image: image_inner_bar.clone(),
                            image_mode: NodeImageMode::Sliced(slicer.clone()),
                            ..default()
                        },
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            // margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ));
            }
        });
}

fn on_game_exit(mut commands: Commands, query: Query<Entity, With<InGame>>)
{
    for entity in query.iter()
    {
        commands.entity(entity).despawn();
    }
}
#[derive(Resource, Component, Default, Clone)]
struct GlassChannel;

#[derive(Resource, Component, Default, Clone)]
struct TurboChannel1p1;
#[derive(Resource, Component, Default, Clone)]
struct TurboChannel2p1;
#[derive(Resource, Component, Default, Clone)]
struct TurboChannel1p2;
#[derive(Resource, Component, Default, Clone)]
struct TurboChannel2p2;
#[derive(Resource, Component, Default, Clone)]
struct TurboChannel1p3;
#[derive(Resource, Component, Default, Clone)]
struct TurboChannel2p3;
#[derive(Resource, Component, Default, Clone)]
struct TurboChannel1p4;
#[derive(Resource, Component, Default, Clone)]
struct TurboChannel2p4;

#[derive(Resource, Component, Default, Clone)]
struct PlayerChannel;

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
    app.add_audio_channel::<GlassChannel>();
    app.add_audio_channel::<TurboChannel1p1>();
    app.add_audio_channel::<TurboChannel2p1>();
    app.add_audio_channel::<TurboChannel1p2>();
    app.add_audio_channel::<TurboChannel2p2>();
    app.add_audio_channel::<TurboChannel1p3>();
    app.add_audio_channel::<TurboChannel2p3>();
    app.add_audio_channel::<TurboChannel1p4>();
    app.add_audio_channel::<TurboChannel2p4>();
    app.add_audio_channel::<PlayerChannel>();

    // cfg_if::cfg_if! {
    //     if #[cfg(not(target_arch = "wasm32"))] {
    //         app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    //     }
    // }
    app.add_systems(Startup, setup);

    app.add_systems(OnEnter(MyAppState::InGame), resetup);
    app.add_systems(OnEnter(MyAppState::InGame), setup_game_player);
    app.add_systems(OnEnter(MyAppState::InGame), setup_glasses);
    app.add_systems(OnEnter(MyAppState::InGame), setup_ui);

    app.add_systems(Update, update_camera.run_if(in_state(MyAppState::InGame)));

    app.add_systems(
        FixedUpdate,
        (use_turbo, drag_force, update_health).run_if(in_state(MyAppState::InGame)),
    );

    app.add_systems(
        FixedPostUpdate,
        (update_ui, try_kill_bubbles, try_kill_by_health).run_if(in_state(MyAppState::InGame)),
    );
    app.add_systems(OnExit(MyAppState::InGame), on_game_exit);

    app.add_systems(
        Update,
        (player_hit_wall, player_hit_player, play_turbo_sound1)
            .run_if(in_state(MyAppState::InGame)),
    );

    // app.add_systems(OnEnter(MyAppState::InGame), start_background_audio);

    app.run();
}
