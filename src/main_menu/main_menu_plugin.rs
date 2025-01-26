use bevy::{
    app::{AppExit, Plugin, Update},
    asset::AssetServer,
    color::{Alpha, Color},
    prelude::{
        in_state, BuildChildren, Button, Changed, ChildBuild, ChildBuilder, Commands, Component,
        DespawnRecursiveExt, Entity, EntityCommands, EventWriter, ImageNode, IntoSystemConfigs,
        NextState, OnEnter, OnExit, Query, Res, ResMut, Text, With,
    },
    text::{TextColor, TextFont},
    ui::{
        AlignContent, AlignItems, BackgroundColor, BorderColor, BorderRadius, BoxShadow,
        FlexDirection, Interaction, JustifyContent, Node, UiRect, Val,
    },
    utils::default,
};

use crate::{AppState, MainMenuState, PlayerNumber};

use super::{
    BORDER_COLOR, BORDER_PX, BORDER_RADIUS_PIXEL, BUTTON_COLOR, BUTTON_HOVER_COLOR, MENU_COLOR,
    TEXT_COLOR,
};

#[derive(Component)]
struct MenuCanvas;

#[derive(Component)]
enum PlayerMenuButton {
    Training,
    Two_Player,
    Three_Player,
    Four_Player,
    Back,
}

#[derive(Component)]
enum HelpMenu {
    BackButton,
    HelpImage,
}

#[derive(Component)]
enum CreditMenu {
    BackButton,
    HelpImage,
}

#[derive(Component)]
enum HomeMenuButton {
    Help,
    Credit,
    Start,
    Quit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu);
        app.add_systems(
            Update,
            (
                button_render_system,
                button_on_press_home_system,
                button_on_press_players_system,
                button_on_press_help_system,
                button_on_press_credit_system,
            )
                .run_if(in_state(AppState::MainMenu)),
        );
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu_fully);
        app.add_systems(OnExit(MainMenuState::HomeMenu), despawn_home_menu);
        app.add_systems(OnExit(MainMenuState::PlayerMenu), despawn_player_menu);
        app.add_systems(OnExit(MainMenuState::Help), despawn_help_menu);
        app.add_systems(OnExit(MainMenuState::Credit), despawn_credit_menu);

        app.add_systems(OnEnter(MainMenuState::HomeMenu), spawn_home_menu);
        app.add_systems(OnEnter(MainMenuState::PlayerMenu), spawn_player_menu);
        app.add_systems(OnEnter(MainMenuState::Help), spawn_help_menu);
        app.add_systems(OnEnter(MainMenuState::Credit), spawn_credit_menu);
    }
}

fn create_button<'a, T: Component>(
    parent: &'a mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    button_text: &str,
    menu_button: T,
) -> EntityCommands<'a> {
    let mut binding = parent.spawn((
        Button,
        menu_button,
        Node {
            width: Val::Px(160.0),
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
            margin: UiRect::new(Val::Px(30.), Val::Px(30.), Val::Px(30.), Val::Px(30.)),
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
        MenuCanvas,
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

fn button_on_press_home_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &HomeMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                HomeMenuButton::Start => {
                    menu_state.set(MainMenuState::PlayerMenu);
                    // app_state.set(AppState::InGame);
                    // commands.insert_resource(PlayerNumber(1));
                }
                HomeMenuButton::Help => menu_state.set(MainMenuState::Help),
                HomeMenuButton::Quit => {
                    exit.send(AppExit::Success);
                }
                HomeMenuButton::Credit => menu_state.set(MainMenuState::Credit),
            }
        }
    }
}

fn button_on_press_players_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &PlayerMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                PlayerMenuButton::Training => {
                    app_state.set(AppState::InGame);
                    commands.insert_resource(PlayerNumber(1));
                }
                PlayerMenuButton::Two_Player => {
                    app_state.set(AppState::InGame);
                    commands.insert_resource(PlayerNumber(2));
                }
                PlayerMenuButton::Three_Player => {
                    app_state.set(AppState::InGame);
                    commands.insert_resource(PlayerNumber(3));
                }
                PlayerMenuButton::Four_Player => {
                    app_state.set(AppState::InGame);
                    commands.insert_resource(PlayerNumber(4));
                }
                PlayerMenuButton::Back => menu_state.set(MainMenuState::HomeMenu),
            }
        }
    }
}

fn button_on_press_help_system(
    mut interaction_query: Query<(&Interaction, &HelpMenu), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                HelpMenu::BackButton => menu_state.set(MainMenuState::HomeMenu),
                HelpMenu::HelpImage => (),
            }
        }
    }
}

fn button_on_press_credit_system(
    mut interaction_query: Query<(&Interaction, &CreditMenu), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                CreditMenu::BackButton => menu_state.set(MainMenuState::HomeMenu),
                CreditMenu::HelpImage => (),
            }
        }
    }
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MenuCanvas>>,
) {
    if query.get_single().is_err() {
        let splash = asset_server.load("sprite/Splash_Screen.png");
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_content: AlignContent::End,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    ..default()
                },
                ImageNode {
                    image: splash,
                    ..default()
                },
                BackgroundColor(bevy::color::Color::srgb(0.5, 0.5, 0.5)),
            ))
            .with_children(|parent| {
                create_menu(parent);
            });
    }
}
//////
fn despawn_main_menu_fully(
    query: Query<Entity, With<Node>>, // Query for entities with a `Button` component
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn(); // Despawn entity and its children
    }
}

//////
fn despawn_credit_menu(
    query: Query<Entity, With<CreditMenu>>, // Query for entities with a `Button` component
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive(); // Despawn entity and its children
    }
}

fn despawn_help_menu(
    query: Query<Entity, With<HelpMenu>>, // Query for entities with a `Button` component
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive(); // Despawn entity and its children
    }
}

fn despawn_player_menu(
    query: Query<Entity, With<PlayerMenuButton>>, // Query for entities with a `Button` component
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive(); // Despawn entity and its children
    }
}

fn despawn_home_menu(
    query: Query<Entity, With<HomeMenuButton>>, // Query for entities with a `Button` component
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive(); // Despawn entity and its children
    }
}

/////

fn spawn_credit_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MenuCanvas>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).with_children(|menu_parent| {
            create_button(menu_parent, &asset_server, "Back", CreditMenu::BackButton);
        });
    }
}

fn spawn_help_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MenuCanvas>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).with_children(|menu_parent| {
            create_button(menu_parent, &asset_server, "Back", HelpMenu::BackButton);
        });
    }
}

fn spawn_player_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MenuCanvas>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).with_children(|menu_parent| {
            create_button(
                menu_parent,
                &asset_server,
                "Training",
                PlayerMenuButton::Training,
            );
            create_button(
                menu_parent,
                &asset_server,
                "2 Players",
                PlayerMenuButton::Two_Player,
            );
            create_button(
                menu_parent,
                &asset_server,
                "3 Players",
                PlayerMenuButton::Three_Player,
            );
            create_button(
                menu_parent,
                &asset_server,
                "4 Players",
                PlayerMenuButton::Four_Player,
            );
            create_button(menu_parent, &asset_server, "Back", PlayerMenuButton::Back);
        });
    }
}

fn spawn_home_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MenuCanvas>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).with_children(|menu_parent| {
            create_button(menu_parent, &asset_server, "Start", HomeMenuButton::Start);
            create_button(menu_parent, &asset_server, "Help", HomeMenuButton::Help);
            create_button(menu_parent, &asset_server, "Credit", HomeMenuButton::Credit);
            cfg_if::cfg_if! {
                if #[cfg(not(target_arch = "wasm32"))] {
                    create_button(menu_parent, &asset_server, "Quit", HomeMenuButton::Quit);
                }
            }
        });
    } else {
        let splash = asset_server.load("sprite/Splash_Screen.png");
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_content: AlignContent::End,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    ..default()
                },
                ImageNode {
                    image: splash,
                    ..default()
                },
                BackgroundColor(bevy::color::Color::srgb(0.5, 0.5, 0.5)),
            ))
            .with_children(|parent| {
                create_menu(parent).with_children(|menu_parent| {
                    create_button(menu_parent, &asset_server, "Start", HomeMenuButton::Start);
                    create_button(menu_parent, &asset_server, "Help", HomeMenuButton::Help);
                    create_button(menu_parent, &asset_server, "Credit", HomeMenuButton::Credit);
                    cfg_if::cfg_if! {
                        if #[cfg(not(target_arch = "wasm32"))] {
                            create_button(menu_parent, &asset_server, "Quit", HomeMenuButton::Quit);
                        }
                    }
                });
            });
    }
}
