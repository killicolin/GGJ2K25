use bevy::{
    app::{AppExit, Plugin, Update},
    asset::AssetServer,
    color::{Alpha, Color},
    prelude::{
        in_state, BuildChildren, Button, Changed, ChildBuild, ChildBuilder, Commands, Component,
        Entity, EntityCommands, EventWriter, IntoSystemConfigs, NextState, OnEnter, OnExit, Query,
        Res, ResMut, Text, With,
    },
    text::{TextColor, TextFont},
    ui::{
        AlignContent, AlignItems, BackgroundColor, BorderColor, BorderRadius, BoxShadow,
        FlexDirection, Interaction, JustifyContent, Node, UiRect, Val,
    },
    utils::default,
};

use crate::MyAppState;

use super::{
    BORDER_COLOR, BORDER_PX, BORDER_RADIUS_PIXEL, BUTTON_COLOR, BUTTON_HOVER_COLOR, MENU_COLOR,
    TEXT_COLOR,
};

#[derive(Component)]
enum MenuButton {
    Help,
    Start,
    Quit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(MyAppState::MainMenu), setup_main_menu);
        app.add_systems(
            Update,
            (button_render_system, button_on_press_system).run_if(in_state(MyAppState::MainMenu)),
        );
        app.add_systems(OnExit(MyAppState::MainMenu), despawn_main_menu);
    }
}

fn create_button<'a>(
    parent: &'a mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    button_text: &str,
    menu_button: MenuButton,
) -> EntityCommands<'a> {
    let mut binding = parent.spawn((
        Button,
        menu_button,
        Node {
            width: Val::Px(120.0),
            height: Val::Px(60.0),
            border: UiRect::all(Val::Px(BORDER_PX)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(BORDER_COLOR),
        BorderRadius::new(
            Val::Px(BORDER_RADIUS_PIXEL),
            Val::Px(BORDER_RADIUS_PIXEL),
            Val::Px(BORDER_RADIUS_PIXEL),
            Val::Px(BORDER_RADIUS_PIXEL),
        ),
        BackgroundColor(BUTTON_COLOR),
        BoxShadow {
            color: Color::BLACK.with_alpha(0.8),
            x_offset: Val::Percent(5.),
            y_offset: Val::Percent(10.),
            spread_radius: Val::Percent(0.),
            blur_radius: Val::Px(5.0),
        },
    ));
    binding.with_child((
        Text::new(button_text),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    ));
    binding
}

fn create_menu<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn((
        Node {
            padding: UiRect::all(Val::Px(40.)),
            border: UiRect::all(Val::Px(BORDER_PX)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(30.),
            ..default()
        },
        BorderColor(BORDER_COLOR),
        BorderRadius::new(
            Val::Px(BORDER_RADIUS_PIXEL),
            Val::Px(BORDER_RADIUS_PIXEL),
            Val::Px(BORDER_RADIUS_PIXEL),
            Val::Px(BORDER_RADIUS_PIXEL),
        ),
        BackgroundColor(MENU_COLOR),
        BoxShadow {
            color: Color::BLACK.with_alpha(0.8),
            x_offset: Val::Percent(5.),
            y_offset: Val::Percent(5.),
            spread_radius: Val::Percent(0.),
            blur_radius: Val::Px(5.0),
        },
    ))
}

fn button_render_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_COLOR.into();
            }
            Interaction::Pressed => (),
        }
    }
}

fn button_on_press_system(
    mut interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<MyAppState>>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                MenuButton::Start => app_state.set(MyAppState::InGame),
                MenuButton::Help => todo!(),
                MenuButton::Quit => {
                    exit.send(AppExit::Success);
                }
            }
        }
    }
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bevy::color::Color::srgb(0.5, 0.5, 0.5)),
        ))
        .with_children(|parent| {
            let mut menu_entity = create_menu(parent);
            menu_entity.with_children(|menu_parent| {
                create_button(menu_parent, &asset_server, "Start", MenuButton::Start);
                create_button(menu_parent, &asset_server, "Help", MenuButton::Help);
                cfg_if::cfg_if! {
                    if #[cfg(not(target_arch = "wasm32"))] {
                        create_button(menu_parent, &asset_server, "Quit", MenuButton::Quit);
                    }
                }
            });
        });
}

fn despawn_main_menu(
    query: Query<Entity, With<Node>>, // Query for entities with a `Button` component
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn(); // Despawn entity and its children
    }
}
