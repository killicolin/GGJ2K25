use avian2d::prelude::{Collisions, LinearVelocity};
use bevy::{
    app::{Plugin, Update},
    asset::AssetServer,
    math::Vec2,
    prelude::{in_state, Entity, IntoSystemConfigs, Query, Res, With, Without},
};
use bevy_kira_audio::{AudioChannel, AudioControl};
use rand::Rng;

use crate::{
    my_audio::my_audio_plugin::{GlassChannel1, GlassChannel2, GlassChannel3, GlassChannel4},
    Glass, Health, MyAppState, Player,
};

pub struct OnHitPlugin;

impl Plugin for OnHitPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (player_hit_wall, player_hit_player).run_if(in_state(MyAppState::InGame)),
        );
    }
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
    audio1: Res<AudioChannel<GlassChannel1>>,
    audio2: Res<AudioChannel<GlassChannel2>>,
    audio3: Res<AudioChannel<GlassChannel3>>,
    audio4: Res<AudioChannel<GlassChannel4>>,
    collisions: Res<Collisions>,
    mut query_player: Query<(Entity, &LinearVelocity, &mut Health, &Player), (Without<Glass>)>,
    query_glass: Query<Entity, (With<Glass>, Without<Player>)>,
) {
    for (entity_player, player_velocity, mut heath, player) in &mut query_player {
        for (entity_wall) in &query_glass {
            if let Some(player_clash) = collisions.get(entity_player, entity_wall) {
                let v = player_velocity.0.distance(Vec2::default());
                let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                if v > 60. && !audio1.is_playing_sound() {
                    heath.0 -= f32::min((v / 20.), 20.);
                    match player.0 {
                        0 => {
                            audio1.play(asset_server.load(format!(
                                "audio/Sfx_impactglass{}.wav",
                                rng.gen_range(1..=2)
                            )));
                        }
                        1 => {
                            audio2.play(asset_server.load(format!(
                                "audio/Sfx_impactglass{}.wav",
                                rng.gen_range(1..=2)
                            )));
                        }
                        2 => {
                            audio3.play(asset_server.load(format!(
                                "audio/Sfx_impactglass{}.wav",
                                rng.gen_range(1..=2)
                            )));
                        }
                        3 => {
                            audio4.play(asset_server.load(format!(
                                "audio/Sfx_impactglass{}.wav",
                                rng.gen_range(1..=2)
                            )));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
