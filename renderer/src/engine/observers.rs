use crate::engine::components;
use crate::engine::components::ui;
use bevy::prelude::*;

use crate::engine::components::building::Building;
use crate::engine::components::construct::Construct;
use crate::engine::components::grid::GridPos;
use crate::engine::components::ui::BuildingInfoPanel;
use crate::engine::components::ui::BuildingPanelContent;
use crate::engine::components::ui::BuildingPanelTitle;
use crate::engine::components::ui::ConstructInfoPanel;
use crate::engine::components::ui::ConstructPanelContent;
use crate::engine::components::ui::ConstructPanelTitle;
use crate::engine::components::ui::UiSelectionBeam;

/// System param grouping for buildings and construct that have the selected component.
#[derive(bevy::ecs::system::SystemParam)]
struct SelectedObjects<'w, 's> {
    selected_buildings: Query<'w, 's, Entity, With<components::building::SelectedBuilding>>,
    selected_constructs: Query<'w, 's, Entity, With<components::construct::SelectedConstruct>>,
}

/// System param to access all the building ui related components
#[derive(bevy::ecs::system::SystemParam)]
struct SelectedBuildingUi<'w, 's> {
    /// Visibility component of the main UI panel for the buildings.
    building_info_panel_visibility: Single<'w, 's, &mut Visibility, (With<ui::BuildingInfoPanel>, Without<ui::UiSelectionBeam>)>,
    /// Title of the UI component of selected buildings.
    building_info_panel_title: Single<'w, 's, &mut Text, (With<ui::BuildingPanelTitle>, Without<ui::BuildingPanelContent>)>,
    /// Content of the UI component of selected buildings.
    building_info_panel_content: Single<'w, 's, &mut Text, (With<ui::BuildingPanelContent>, Without<ui::BuildingPanelTitle>)>,
    /// Visibility and transform of the beam that shoots out of the selected building.
    selection_beam: Single<'w, 's, (&mut Transform, &mut Visibility), With<ui::UiSelectionBeam>>,
}

impl<'w, 's> SelectedBuildingUi<'w, 's> {
    fn show(&mut self, title: String, content: String, beam_pos: Vec3) {
        let (selection_beam_visibility, selection_beam_tf) = &mut *self.selection_beam;

        *self.building_info_panel_visibility = Visibility::Visible;
        selection_beam_visibility = Visibility::Visible;

        selection_beam_tf = beam_pos;

        self.building_info_panel_title.0 = title;
        self.building_info_panel_content.0 = content;
    }
}

/// Observer for when a building gets selected.
pub fn on_building_clicked(click: On<Pointer<Click>>, selected: SelectedObjects, mut commands: Commands) {
    /* Deselect all selected buildings / constructs */
    for selected_building in selected_buildings {
        commands.entity(selected_building).remove::<SelectedBuilding>();
    }
    for selected_construct in selected_constructs {
        commands.entity(selected_construct).remove::<SelectedConstruct>();
    }

    /* Set the newly selected entity as selected */
    commands.entity(click.entity).insert(SelectedBuilding);

    /* Set the UI as visible */
    let (visibility, _) = &mut *building_info_panel;
    **visibility = bevy::prelude::Visibility::Visible;

    /* Set the info panel with the correct info and position */
    let (title, _) = &mut *building_info_panel_title;
    let (content, _) = &mut *building_info_panel_content;
    title.0 = building.building.kind.to_string();
    content.0 = format!(
        "A cute lil house.\nCreated the {}",
        building.building.created_at.format("%Y-%m-%d at %H:%M")
    );

    /* Set the selection beam visible and at the right place */
    let (beam_tf, beam_vis) = &mut *selection_beam;
    let beam_grid_pos = GridPos {
        x: building.building.pos.x,
        y: building.building.pos.y,
    };
    beam_tf.translation = beam_grid_pos.world() + bevy::prelude::Vec3::Y * 0.6;
    **beam_vis = bevy::prelude::Visibility::Visible;

    click.propagate(false);
}

/// Observer for when a construct gets selected.
pub fn on_construct_clicked(
    mut click: bevy::prelude::On<bevy::prelude::Pointer<bevy::prelude::Click>>,
    constructs: bevy::prelude::Query<&Construct>,
    selected_buildings: bevy::prelude::Query<bevy::prelude::Entity, bevy::prelude::With<SelectedBuilding>>,
    selected_constructs: bevy::prelude::Query<bevy::prelude::Entity, bevy::prelude::With<SelectedConstruct>>,
    mut construct_info_panel: bevy::prelude::Single<
        (&mut bevy::prelude::Visibility, &ConstructInfoPanel),
        bevy::prelude::Without<UiSelectionBeam>,
    >,
    mut construct_info_panel_title: bevy::prelude::Single<
        (&mut bevy::prelude::Text, &ConstructPanelTitle),
        bevy::prelude::With<ConstructPanelTitle>,
    >,
    mut construct_info_panel_content: bevy::prelude::Single<
        (&mut bevy::prelude::Text, &ConstructPanelContent),
        bevy::prelude::Without<ConstructPanelTitle>,
    >,
    mut selection_beam: bevy::prelude::Single<
        (&mut bevy::prelude::Transform, &mut bevy::prelude::Visibility),
        bevy::prelude::With<UiSelectionBeam>,
    >,
    mut commands: bevy::prelude::Commands,
) {
    /* Get the construct component on the clicked entity, otherwise, discard */
    let Ok(construct) = constructs.get(click.entity) else { return };

    /* Deselect all selected buildings / constructs */
    for selected_building in selected_buildings {
        commands.entity(selected_building).remove::<SelectedBuilding>();
    }
    for selected_construct in selected_constructs {
        commands.entity(selected_construct).remove::<SelectedConstruct>();
    }

    /* Set the newly selected entity as selected */
    commands.entity(click.entity).insert(SelectedConstruct);

    /* Set the UI as visible */
    let (visibility, _) = &mut *construct_info_panel;
    **visibility = bevy::prelude::Visibility::Visible;

    /* Set the info panel with the correct info and position */
    let (title, _) = &mut *construct_info_panel_title;
    let (content, _) = &mut *construct_info_panel_content;
    title.0 = construct.building.kind.to_string();
    content.0 = format!(
        "A cute lil house.\nVotes: {} / {}\nAsked the {}",
        construct.status.votes,
        construct.status.cost,
        construct.building.created_at.format("%Y-%m-%d at %H:%M")
    );

    /* Set the selection beam visible and at the right place */
    let (beam_tf, beam_vis) = &mut *selection_beam;
    let beam_grid_pos = GridPos {
        x: construct.building.pos.x,
        y: construct.building.pos.y,
    };
    beam_tf.translation = beam_grid_pos.world() + bevy::prelude::Vec3::Y * 0.6;
    **beam_vis = bevy::prelude::Visibility::Visible;

    click.propagate(false);
}
