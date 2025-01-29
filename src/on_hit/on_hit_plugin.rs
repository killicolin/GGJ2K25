use avian2d::prelude::{Collisions, LinearVelocity};
use bevy::{
    app::{Plugin, Update},
    math::Vec2,
    prelude::{in_state, Entity, IntoSystemConfigs, Query, Res, With, Without},
};
use bevy_kira_audio::{AudioChannel, AudioControl};
use rand::Rng;

use crate::{
    my_audio::my_audio_plugin::{AudioAssets, GlassChannel, PlayerChannel},
    AppState, Glass, Health, Player,
};

pub struct OnHitPlugin;

impl Plugin for OnHitPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (player_hit_wall, player_hit_player).run_if(in_state(AppState::InGame)),
        );
    }
}

fn player_hit_player(
    audio_server: Res<AudioAssets>,
    collisions: Res<Collisions>,
    audio: Res<AudioChannel<PlayerChannel>>,
    mut query: Query<(&LinearVelocity, Entity, &mut Health), (With<Player>)>,
) {
    let _ = audio;
    let mut combinations = query.iter_combinations_mut();
    while let Some([c1, c2]) = combinations.fetch_next() {
        let (velocity1, e1, mut h1) = c1;
        let (velocity2, e2, mut h2) = c2;
        if let Some(player_clash) = collisions.get(e1, e2) {
            let v1 = velocity1.0.distance(Vec2::default());
            let v2 = velocity2.0.distance(Vec2::default());
            let mut rng: rand::prelude::ThreadRng = rand::rng();
            if player_clash.collision_started() {
                println!("{} {} {}", v1, v2, player_clash.total_normal_impulse);
                h1.0 -= f32::min(v2 / 10., 20.);
                h2.0 -= f32::min(v1 / 10., 20.);
                let sound = match rng.random_range(1..=3) {
                    1 => &audio_server.tabshock_1,
                    2 => &audio_server.tabshock_2,
                    _ => &audio_server.tabshock_3,
                };
                audio.play(sound.clone());
            }
        }
    }
}

fn player_hit_wall(
    audio_server: Res<AudioAssets>,
    audio: Res<AudioChannel<GlassChannel>>,
    collisions: Res<Collisions>,
    mut query_player: Query<(Entity, &LinearVelocity, &mut Health), (Without<Glass>)>,
    query_glass: Query<Entity, (With<Glass>, Without<Player>)>,
) {
    for (entity_player, player_velocity, mut heath) in &mut query_player {
        for (entity_wall) in &query_glass {
            if let Some(player_clash) = collisions.get(entity_player, entity_wall) {
                let v = player_velocity.0.distance(Vec2::default());
                let mut rng: rand::prelude::ThreadRng = rand::rng();
                if player_clash.collision_started() {
                    heath.0 -= f32::min((v / 20.), 20.);
                    let sound = match rng.random_range(1..=2) {
                        1 => &audio_server.impact_glass_1,
                        _ => &audio_server.impact_glass_2,
                    };
                    audio.play(sound.clone());
                }
            }
        }
    }
}
