use bevy::prelude::*;

use crate::engine::components::ui::BuildingInfoPanel;
use crate::engine::components::ui::BuildingPanelContent;
use crate::engine::components::ui::BuildingPanelTitle;
use crate::engine::components::ui::ButtonKind;
use crate::engine::components::ui::ConstructInfoPanel;
use crate::engine::components::ui::ConstructPanelContent;
use crate::engine::components::ui::ConstructPanelTitle;
use crate::engine::components::ui::HiddableSelectionMarker;
use crate::engine::components::ui::UiSelectionBeam;

/// Color for the background panels of the UI.
const UI_BACKGROUND_COLOR: Color = Color::srgba(0.1, 0.1, 0.12, 0.98);
/// Color for the background panels of the UI.
const UI_BORDER_COLOR: Color = Color::WHITE;
/// Color for the background panels of the UI.
const UI_BORDER_SUBTLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.32);
/// Color for the titles in UI.
const UI_TITLE_COLOR: Color = Color::WHITE;
/// Color for text in UI.
const UI_TEXT_COLOR: Color = Color::srgb(0.8, 0.8, 0.9);

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    setup_scene(&mut commands);
    setup_ui(&mut commands, &mut meshes, &mut materials);
}

fn setup_scene(commands: &mut Commands) {
    /* Little camera to see the world */
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            /* 6 world units per pixel of window height. */
            scaling_mode: bevy::camera::ScalingMode::FixedVertical { viewport_height: 15.0 },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(8.0, 6.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    /* Cute lil light */
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_ui(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) {
    /* UI panel for buildings */
    commands.spawn((
        BuildingInfoPanel,
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            padding: UiRect::axes(px(10.0), px(6.0)),
            flex_direction: FlexDirection::Column,
            border_radius: BorderRadius::all(px(6.0)),
            border: UiRect::all(px(1.0)),
            ..default()
        },
        BackgroundColor(UI_BACKGROUND_COLOR),
        BorderColor {
            top: UI_BORDER_COLOR,
            bottom: UI_BORDER_COLOR,
            left: UI_BORDER_COLOR,
            right: UI_BORDER_COLOR,
        },
        Visibility::Hidden,
        HiddableSelectionMarker,
        children![
            (
                BuildingPanelTitle,
                Text::new(""),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(UI_TITLE_COLOR),
            ),
            (
                BuildingPanelContent,
                Text::new(""),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(UI_TEXT_COLOR),
            ),
        ],
    ));

    /* UI panel for constructs */
    commands.spawn((
        ConstructInfoPanel,
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            padding: UiRect::axes(px(10.0), px(6.0)),
            flex_direction: FlexDirection::Column,
            border_radius: BorderRadius::all(px(6.0)),
            border: UiRect::all(px(1.0)),
            ..default()
        },
        BackgroundColor(UI_BACKGROUND_COLOR),
        BorderColor {
            top: UI_BORDER_COLOR,
            bottom: UI_BORDER_COLOR,
            left: UI_BORDER_COLOR,
            right: UI_BORDER_COLOR,
        },
        Visibility::Hidden,
        HiddableSelectionMarker,
        children![
            (
                ConstructPanelTitle,
                Text::new(""),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(UI_TITLE_COLOR),
            ),
            (
                ConstructPanelContent,
                Text::new(""),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(UI_TEXT_COLOR),
            ),
            panel_button("Upvote building", ButtonKind::UpvoteBuilding),
            panel_button("Downvote building", ButtonKind::DownvoteBuilding),
        ],
    ));

    commands.spawn((
        UiSelectionBeam,
        Mesh3d(meshes.add(Cuboid::new(0.02, 1.2, 0.02))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: UI_BORDER_COLOR,
            unlit: true,
            ..default()
        })),
        Transform::default(),
        Visibility::Hidden,
        HiddableSelectionMarker,
        Pickable::IGNORE,
    ));
}

fn panel_button(label: &str, button_kind: ButtonKind) -> impl Bundle {
    (
        Button,
        button_kind,
        Node {
            padding: UiRect::axes(px(8.0), px(4.0)),
            border: UiRect::all(px(1.0)),
            border_radius: BorderRadius::all(px(2.0)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(UI_BACKGROUND_COLOR),
        BorderColor {
            top: UI_BORDER_SUBTLE_COLOR,
            bottom: UI_BORDER_SUBTLE_COLOR,
            left: UI_BORDER_SUBTLE_COLOR,
            right: UI_BORDER_SUBTLE_COLOR,
        },
        children![(
            Text::new(label),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(UI_TEXT_COLOR),
        )],
    )
}
