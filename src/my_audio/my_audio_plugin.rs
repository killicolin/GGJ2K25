use bevy::{
    app::{Plugin, Update},
    asset::{AssetServer, Handle},
    input::ButtonInput,
    prelude::{
        in_state, Component, IntoSystemConfigs, KeyCode, OnEnter, Query, Res, Resource, Transform,
    },
};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_kira_audio::{Audio, AudioApp, AudioChannel, AudioControl, AudioSource};

use crate::{is_in_water, AppState, Player, PLAYER_CONTROL};

pub struct MyAudioPlugin;

#[derive(Resource, Component, Default, Clone)]
pub struct GlassChannel;

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
pub struct PlayerChannel;

fn play_menu_music(audio_assets: Res<AudioAssets>, audio: Res<AudioChannel<SongChannel>>) {
    if audio.is_playing_sound() {
        audio.stop();
    }
    audio
        .play(audio_assets.in_menu_theme.clone())
        .with_volume(1.0)
        .looped();
}

fn play_game_music(audio_assets: Res<AudioAssets>, audio: Res<AudioChannel<SongChannel>>) {
    if audio.is_playing_sound() {
        audio.stop();
    }
    audio
        .play(audio_assets.in_game_theme.clone())
        .with_volume(1.5)
        .looped();
}

fn play_effervescent_sound(
    audio_assets: Res<AudioAssets>,
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
                            .play(audio_assets.effer_1.clone())
                            .with_volume(0.08)
                            .looped();
                    }
                }
                1 => {
                    if !audio2.is_playing_sound() {
                        audio2
                            .play(audio_assets.effer_2.clone())
                            .with_volume(0.08)
                            .looped();
                    }
                }
                2 => {
                    if !audio3.is_playing_sound() {
                        audio3
                            .play(audio_assets.effer_1.clone())
                            .with_volume(0.08)
                            .looped();
                    }
                }
                3 => {
                    if !audio4.is_playing_sound() {
                        audio4
                            .play(audio_assets.effer_2.clone())
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

fn play_turbo_sound<T: Resource, P: Resource>(
    audio_assets: &Res<AudioAssets>,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    audio: &Res<Audio>,
    audio1: &Res<AudioChannel<T>>,
    audio2: &Res<AudioChannel<P>>,
    transform: &Transform,
    player: &Player,
) {
    if is_in_water(&transform.translation) {
        if keyboard_input.pressed(PLAYER_CONTROL[player.0].up)
            || keyboard_input.pressed(PLAYER_CONTROL[player.0].down)
            || (keyboard_input.pressed(PLAYER_CONTROL[player.0].left)
                && keyboard_input.pressed(PLAYER_CONTROL[player.0].right))
        {
            if !audio1.is_playing_sound() {
                audio.play(audio_assets.boost_explosion.clone());
                audio1
                    .play(audio_assets.boost_1.clone())
                    .loop_from(0.75)
                    .with_volume(0.5);
            }
            if !audio2.is_playing_sound() {
                audio.play(audio_assets.boost_explosion.clone());
                audio2
                    .play(audio_assets.boost_2.clone())
                    .loop_from(0.75)
                    .with_volume(0.5);
            }
        } else if keyboard_input.pressed(PLAYER_CONTROL[player.0].left) {
            if !audio1.is_playing_sound() {
                audio.play(audio_assets.boost_explosion.clone());
                audio1
                    .play(audio_assets.boost_1.clone())
                    .loop_from(0.75)
                    .with_volume(0.5);
            }
            if audio2.is_playing_sound() {
                audio2.stop();
            }
        } else if keyboard_input.pressed(PLAYER_CONTROL[player.0].right) {
            if !audio2.is_playing_sound() {
                audio.play(audio_assets.boost_explosion.clone());
                audio2
                    .play(audio_assets.boost_2.clone())
                    .loop_from(0.75)
                    .with_volume(0.5);
            }
            if audio1.is_playing_sound() {
                audio1.stop();
            }
        } else {
            if audio1.is_playing_sound() {
                audio1.stop();
            }
            if audio2.is_playing_sound() {
                audio2.stop();
            }
        }
    } else {
        if audio1.is_playing_sound() {
            audio1.stop();
        }
        if audio2.is_playing_sound() {
            audio2.stop();
        }
    }
}

fn play_turbo_sound1(
    audio_assets: Res<AudioAssets>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio: Res<Audio>,
    audio1: Res<AudioChannel<TurboChannel1p1>>,
    audio2: Res<AudioChannel<TurboChannel2p1>>,
    in_water_object: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &in_water_object {
        if player.0 == 0 {
            play_turbo_sound::<TurboChannel1p1, TurboChannel2p1>(
                &audio_assets,
                &keyboard_input,
                &audio,
                &audio1,
                &audio2,
                transform,
                player,
            );
            break;
        }
    }
}

fn play_turbo_sound2(
    audio_assets: Res<AudioAssets>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio: Res<Audio>,
    audio1: Res<AudioChannel<TurboChannel1p2>>,
    audio2: Res<AudioChannel<TurboChannel2p2>>,
    in_water_object: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &in_water_object {
        if player.0 == 1 {
            play_turbo_sound::<TurboChannel1p2, TurboChannel2p2>(
                &audio_assets,
                &keyboard_input,
                &audio,
                &audio1,
                &audio2,
                transform,
                player,
            );
            break;
        }
    }
}

fn play_turbo_sound3(
    audio_assets: Res<AudioAssets>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio: Res<Audio>,
    audio1: Res<AudioChannel<TurboChannel1p3>>,
    audio2: Res<AudioChannel<TurboChannel2p3>>,
    in_water_object: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &in_water_object {
        if player.0 == 2 {
            play_turbo_sound::<TurboChannel1p3, TurboChannel2p3>(
                &audio_assets,
                &keyboard_input,
                &audio,
                &audio1,
                &audio2,
                transform,
                player,
            );
            break;
        }
    }
}

fn play_turbo_sound4(
    audio_assets: Res<AudioAssets>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio: Res<Audio>,
    audio1: Res<AudioChannel<TurboChannel1p4>>,
    audio2: Res<AudioChannel<TurboChannel2p4>>,
    in_water_object: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &in_water_object {
        if player.0 == 3 {
            play_turbo_sound::<TurboChannel1p4, TurboChannel2p4>(
                &audio_assets,
                &keyboard_input,
                &audio,
                &audio1,
                &audio2,
                transform,
                player,
            );
            break;
        }
    }
}

impl Plugin for MyAudioPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_audio_channel::<SongChannel>();
        app.add_audio_channel::<GlassChannel>();
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

        app.add_systems(OnEnter(AppState::InGame), play_game_music);
        app.add_systems(OnEnter(AppState::MainMenu), play_menu_music);

        app.add_systems(
            Update,
            (
                play_turbo_sound1,
                play_turbo_sound2,
                play_turbo_sound3,
                play_turbo_sound4,
                play_effervescent_sound,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/Music_Les petits effervescents v1.mp3")]
    in_game_theme: Handle<AudioSource>,
    #[asset(path = "audio/Musicmenu_Les petits effervescents.wav")]
    in_menu_theme: Handle<AudioSource>,

    #[asset(path = "audio/Sfx_effer1.wav")]
    effer_1: Handle<AudioSource>,
    #[asset(path = "audio/Sfx_effer2.wav")]
    effer_2: Handle<AudioSource>,

    #[asset(path = "audio/Sfx_boost1.wav")]
    boost_1: Handle<AudioSource>,
    #[asset(path = "audio/Sfx_boost2.wav")]
    boost_2: Handle<AudioSource>,
    #[asset(path = "audio/Sfx_boostExplosion.wav")]
    boost_explosion: Handle<AudioSource>,

    #[asset(path = "audio/Sfx_tabshock1.wav")]
    pub tabshock_1: Handle<AudioSource>,
    #[asset(path = "audio/Sfx_tabshock2.wav")]
    pub tabshock_2: Handle<AudioSource>,
    #[asset(path = "audio/Sfx_tabshock3.wav")]
    pub tabshock_3: Handle<AudioSource>,

    #[asset(path = "audio/Sfx_impactglass1.wav")]
    pub impact_glass_1: Handle<AudioSource>,
    #[asset(path = "audio/Sfx_impactglass2.wav")]
    pub impact_glass_2: Handle<AudioSource>,
}
