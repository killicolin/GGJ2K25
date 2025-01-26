use std::usize;

use bevy::{
    app::{FixedPostUpdate, Plugin},
    asset::AssetServer,
    color::Color,
    prelude::{
        in_state, BuildChildren, ChildBuild, Commands, ImageNode, IntoSystemConfigs, OnEnter,
        Query, Res, With, Without,
    },
    sprite::{BorderRect, SliceScaleMode, TextureSlicer},
    ui::{widget::NodeImageMode, AlignItems, Display, JustifyContent, Node, UiRect, Val},
    utils::default,
};

use crate::{
    Health, HudInnerBar, HudPlayer, InGame, AppState, Player, PlayerNumber, INITIAL_HEALTH,
    PLAYER_COLOR,
};

pub struct GameHudPlugin;

impl Plugin for GameHudPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui);
        app.add_systems(
            FixedPostUpdate,
            (update_ui).run_if(in_state(AppState::InGame)),
        );
    }
}

fn update_ui(
    mut query_players: Query<(&Health, &Player)>,
    mut query_ui_inner: Query<(&mut Node, &HudPlayer), With<HudInnerBar>>,
    mut query_ui_outer: Query<(&mut Node, &HudPlayer), Without<HudInnerBar>>,
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
    asset_server: Res<AssetServer>,
    player_number: Res<PlayerNumber>,
) {
    let image_outer_bar = asset_server.load("sprite/bar_outer.png");
    let image_inner_bar = asset_server.load("sprite/bar_inner.png");

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
            for i in (0..player_number.0) {
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
