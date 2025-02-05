use bevy::{
    app::{FixedPostUpdate, Plugin},
    color::Color,
    prelude::{
        in_state, BuildChildren, ChildBuild, Commands, Entity, ImageNode, IntoSystemConfigs,
        NextState, OnEnter, Query, Res, ResMut, With,
    },
    sprite::{BorderRect, SliceScaleMode, TextureSlicer},
    ui::{widget::NodeImageMode, AlignItems, FlexDirection, JustifyContent, Node, UiRect, Val},
    utils::default,
};

use crate::{
    AppState, EndGameDisplay, Health, HudInnerBar, HudPlayer, InGame, MainMenuState, Player,
    PlayerNumber, SpriteAssets, INITIAL_HEALTH, MENU_DURATION, PLAYER_COLOR,
};

pub struct GameHudPlugin;

impl Plugin for GameHudPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui);
        app.add_systems(
            FixedPostUpdate,
            (update_ui, end_game_display).run_if(in_state(AppState::InGame)),
        );
    }
}

fn end_game_display(
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
    player_number: Res<PlayerNumber>,
    query: Query<(&Health, &Player)>,
    mut query_end_menu: Query<(Entity, &mut EndGameDisplay)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (entity, mut end_menu_display) in query_end_menu.iter_mut() {
        end_menu_display.0 -= 1;
        if end_menu_display.0 == 0 {
            commands.entity(entity).despawn();
            app_state.set(AppState::MainMenu);
            menu_state.set(MainMenuState::HomeMenu);
        }
    }

    if !query_end_menu.is_empty() {
        return;
    }

    let mut last_player_id = 0;
    let mut alive_players = 0;
    for (health, player) in query.iter() {
        if health.0 > 0. {
            alive_players += 1;
            last_player_id = player.0;
        }
    }

    if (player_number.0 == 1 && alive_players <= 0) || (player_number.0 != 1 && alive_players <= 1)
    {
        let mut cup_file = &sprite_assets.cup;
        if alive_players == 0 {
            cup_file = &sprite_assets.cup_dead;
        }
        let image_cup = cup_file.clone();

        let files = [
            &sprite_assets.p1_won,
            &sprite_assets.p2_won,
            &sprite_assets.p3_won,
            &sprite_assets.p4_won,
        ];

        let image_winner = files[last_player_id].clone();

        let slicer = TextureSlicer {
            border: BorderRect::square(64.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };
        commands
            .spawn((
                InGame,
                EndGameDisplay(MENU_DURATION),
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    // top: Val::Percent(86.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    InGame,
                    ImageNode {
                        image: image_cup,
                        image_mode: NodeImageMode::Sliced(slicer.clone()),
                        ..default()
                    },
                    Node {
                        width: Val::Px(400.),
                        aspect_ratio: Some(1.0),
                        ..default()
                    },
                ));

                if alive_players >= 1 {
                    parent.spawn((
                        InGame,
                        ImageNode {
                            image: image_winner,
                            image_mode: NodeImageMode::Stretch,
                            color: Color::from(PLAYER_COLOR[last_player_id]),
                            ..default()
                        },
                        Node {
                            width: Val::Px(400.),
                            aspect_ratio: Some(2.0),
                            ..default()
                        },
                    ));
                }
            });
    }
}

fn update_ui(
    mut query_players: Query<(&Health, &Player)>,
    mut query_ui_inner: Query<(&mut Node, &HudPlayer), With<HudInnerBar>>,
) {
    for (health, player) in &mut query_players {
        for (mut node, hudplayer) in &mut query_ui_inner {
            if hudplayer.0 == player.0 {
                let min = 13.;
                node.width = Val::Percent(min + (100. - min) * health.0.max(0.) / INITIAL_HEALTH);
            }
        }
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_sprite: Res<SpriteAssets>,
    player_number: Res<PlayerNumber>,
) {
    let image_outer_bar = asset_sprite.bar_outer.clone();
    let image_inner_bar = asset_sprite.bar_inner.clone();

    let slicer = TextureSlicer {
        border: BorderRect::square(64.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };
    commands
        .spawn((
            InGame,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(12.0),
                top: Val::Percent(86.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
        ))
        .with_children(|parent| {
            let w = 20.0;
            let h = 32.0;
            for i in 0..player_number.0 {
                parent
                    .spawn((
                        InGame,
                        HudPlayer(i),
                        ImageNode {
                            image: image_outer_bar.clone(),
                            image_mode: NodeImageMode::Sliced(slicer.clone()),
                            color: Color::from(PLAYER_COLOR[i]),
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
                        HudInnerBar,
                        HudPlayer(i),
                        ImageNode {
                            image: image_inner_bar.clone(),
                            image_mode: NodeImageMode::Sliced(slicer.clone()),
                            color: Color::from(PLAYER_COLOR[i]),
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
