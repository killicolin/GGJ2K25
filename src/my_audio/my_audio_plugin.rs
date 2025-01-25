use bevy::{
    app::{Plugin, Update},
    asset::AssetServer,
    input::ButtonInput,
    prelude::{
        in_state, Component, IntoSystemConfigs, KeyCode, OnEnter, Query, Res, Resource, Transform,
    },
};
use bevy_kira_audio::{Audio, AudioApp, AudioChannel, AudioControl};

use crate::{is_in_water, MyAppState, Player};

pub struct MyAudioPlugin;

#[derive(Resource, Component, Default, Clone)]
pub struct GlassChannel1;
#[derive(Resource, Component, Default, Clone)]
pub struct GlassChannel2;
#[derive(Resource, Component, Default, Clone)]
pub struct GlassChannel3;
#[derive(Resource, Component, Default, Clone)]
pub struct GlassChannel4;

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
struct EffervescentChannelp1;
#[derive(Resource, Component, Default, Clone)]
struct EffervescentChannelp2;
#[derive(Resource, Component, Default, Clone)]
struct EffervescentChannelp3;
#[derive(Resource, Component, Default, Clone)]
struct EffervescentChannelp4;

#[derive(Resource, Component, Default, Clone)]
struct SongChannel;

#[derive(Resource, Component, Default, Clone)]
struct PlayerChannel;

fn play_music(asset_server: Res<AssetServer>, audio: Res<AudioChannel<SongChannel>>) {
    if !audio.is_playing_sound() {
        audio
            .play(asset_server.load("audio/Music_Les petits effervescents v1.mp3"))
            .with_volume(2.0)
            .looped();
    }
}

fn play_effervescent_sound(
    asset_server: Res<AssetServer>,
    audio1: Res<AudioChannel<EffervescentChannelp1>>,
    audio2: Res<AudioChannel<EffervescentChannelp2>>,
    audio3: Res<AudioChannel<EffervescentChannelp3>>,
    audio4: Res<AudioChannel<EffervescentChannelp4>>,
    in_water_object: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &in_water_object {
        if is_in_water(&transform.translation) {
            match player.0 {
                0 => {
                    if !audio1.is_playing_sound() {
                        audio1
                            .play(asset_server.load("audio/Sfx_effer1.wav"))
                            .with_volume(0.08)
                            .looped();
                    }
                }
                1 => {
                    if !audio2.is_playing_sound() {
                        audio2
                            .play(asset_server.load("audio/Sfx_effer2.wav"))
                            .with_volume(0.08)
                            .looped();
                    }
                }
                2 => {
                    if !audio3.is_playing_sound() {
                        audio3
                            .play(asset_server.load("audio/Sfx_effer1.wav"))
                            .with_volume(0.08)
                            .looped();
                    }
                }
                3 => {
                    if !audio4.is_playing_sound() {
                        audio4
                            .play(asset_server.load("audio/Sfx_effer2.wav"))
                            .with_volume(0.08)
                            .looped();
                    }
                }
                _ => (),
            }
        } else {
            match player.0 {
                0 => {
                    if audio1.is_playing_sound() {
                        audio1.stop();
                    }
                }
                1 => {
                    if audio2.is_playing_sound() {
                        audio2.stop();
                    }
                }
                2 => {
                    if audio3.is_playing_sound() {
                        audio3.stop();
                    }
                }
                3 => {
                    if audio4.is_playing_sound() {
                        audio4.stop();
                    }
                }
                _ => (),
            }
        }
    }
}

fn play_turbo_sound1(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio: Res<Audio>,
    audio1: Res<AudioChannel<TurboChannel1p1>>,
    audio2: Res<AudioChannel<TurboChannel2p1>>,
    in_water_object: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &in_water_object {
        if is_in_water(&transform.translation) && player.0 == 0 {
            if keyboard_input.pressed(KeyCode::KeyD)
                || keyboard_input.pressed(KeyCode::KeyW)
                || keyboard_input.pressed(KeyCode::KeyS)
            {
                if !audio1.is_playing_sound() {
                    audio.play(asset_server.load("audio/Sfx_boostExplosion.wav"));
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
                    audio.play(asset_server.load("audio/Sfx_boostExplosion.wav"));
                    audio2
                        .play(asset_server.load("audio/Sfx_boost2.wav"))
                        .loop_from(1.0);
                }
            } else if audio2.is_playing_sound() {
                audio2.stop();
            }
        } else {
            if audio2.is_playing_sound() {
                audio2.stop();
            }
            if audio1.is_playing_sound() {
                audio1.stop();
            }
        }
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

impl Plugin for MyAudioPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_audio_channel::<GlassChannel1>();
        app.add_audio_channel::<GlassChannel2>();
        app.add_audio_channel::<GlassChannel3>();
        app.add_audio_channel::<GlassChannel4>();
        app.add_audio_channel::<TurboChannel1p1>();
        app.add_audio_channel::<TurboChannel2p1>();
        app.add_audio_channel::<TurboChannel1p2>();
        app.add_audio_channel::<TurboChannel2p2>();
        app.add_audio_channel::<TurboChannel1p3>();
        app.add_audio_channel::<TurboChannel2p3>();
        app.add_audio_channel::<TurboChannel1p4>();
        app.add_audio_channel::<TurboChannel2p4>();
        app.add_audio_channel::<EffervescentChannelp1>();
        app.add_audio_channel::<EffervescentChannelp2>();
        app.add_audio_channel::<EffervescentChannelp3>();
        app.add_audio_channel::<EffervescentChannelp4>();
        app.add_audio_channel::<PlayerChannel>();
        app.add_audio_channel::<SongChannel>();

        app.add_systems(OnEnter(MyAppState::InGame), play_music);

        app.add_systems(
            Update,
            (play_turbo_sound1, play_effervescent_sound).run_if(in_state(MyAppState::InGame)),
        );
    }
}
