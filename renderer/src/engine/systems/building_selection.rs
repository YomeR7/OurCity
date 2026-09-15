use crate::engine::components::building;
use crate::engine::components::construct;
use crate::engine::components::ui;
use bevy::prelude::*;

/// System param grouping for buildings and construct that have the selected component.
#[derive(bevy::ecs::system::SystemParam)]
pub struct SelectedObjects<'w, 's> {
    /// Query for the buildings entities having the `[SelectedBuilding]` component.
    selected_buildings: Query<'w, 's, Entity, With<building::SelectedBuilding>>,
    /// Query for the constructs entities having the `[SelectedConstruct]` component.
    selected_constructs: Query<'w, 's, Entity, With<construct::SelectedConstruct>>,
}

impl<'w, 's> SelectedObjects<'w, 's> {
    /// Deselect all objects, by removing all the selected components.
    pub fn deselect_all(&self, commands: &mut Commands) {
        for selected_building in self.selected_buildings {
            commands.entity(selected_building).remove::<building::SelectedBuilding>();
        }
        for selected_construct in self.selected_constructs {
            commands.entity(selected_construct).remove::<construct::SelectedConstruct>();
        }
    }
}

/// System param to access all the construct ui related components
#[derive(bevy::ecs::system::SystemParam)]
pub struct SelectedUi<'w, 's, PanelMarker, TitleMarker, ContentMarker>
where
    PanelMarker: Component,
    TitleMarker: Component,
    ContentMarker: Component,
{
    /// Visibility component of the main UI panel for the constructs.
    info_panel_visibility: Single<'w, 's, &'static mut Visibility, (With<PanelMarker>, Without<ui::UiSelectionBeam>)>,
    /// Title of the UI component of selected constructs.
    info_panel_title: Single<'w, 's, &'static mut Text, (With<TitleMarker>, Without<ContentMarker>)>,
    /// Content of the UI component of selected constructs.
    info_panel_content: Single<'w, 's, &'static mut Text, (With<ContentMarker>, Without<TitleMarker>)>,
    /// Visibility and transform of the beam that shoots out of the selected construct.
    selection_beam: Single<'w, 's, (&'static mut Visibility, &'static mut Transform), With<ui::UiSelectionBeam>>,
}

impl<'w, 's, PanelMarker, TitleMarker, ContentMarker> SelectedUi<'w, 's, PanelMarker, TitleMarker, ContentMarker>
where
    PanelMarker: Component,
    TitleMarker: Component,
    ContentMarker: Component,
{
    /// Set the building ui panel to visible, and update the title, content, and beam position
    fn show(&mut self, title: String, content: String, beam_pos: Vec3) {
        let (selection_beam_visibility, selection_beam_tf) = &mut *self.selection_beam;

        **self.info_panel_visibility = Visibility::Visible;
        **selection_beam_visibility = Visibility::Visible;

        selection_beam_tf.translation = beam_pos;

        self.info_panel_title.0 = title;
        self.info_panel_content.0 = content;
    }

    /// Hide the selection panel and the beam
    fn hide(&mut self) {
        let (selection_beam_visibility, selection_beam_tf) = &mut *self.selection_beam;

        **self.info_panel_visibility = Visibility::Hidden;
        **selection_beam_visibility = Visibility::Hidden;
    }
}

#[rustfmt::skip]
pub type SelectedBuildingUi<'w, 's> = SelectedUi<'w, 's, ui::BuildingInfoPanel, ui::BuildingPanelTitle, ui::BuildingPanelContent>;
#[rustfmt::skip]
pub type SelectedConstructUi<'w, 's> = SelectedUi<'w, 's, ui::ConstructInfoPanel, ui::ConstructPanelTitle, ui::ConstructPanelContent>;

/// Observer for when a building gets selected.
pub fn on_building_clicked(
    mut click: On<Pointer<Click>>,
    selected: crate::engine::systems::SelectedObjects,
    mut commands: Commands,
) {
    /* Deselect all selected buildings / constructs */
    selected.deselect_all(&mut commands);

    /* Set the newly selected entity as selected */
    commands.entity(click.entity).insert(building::SelectedBuilding);

    click.propagate(false);
}

/// System called when a new `[SelectedBuilding]` component is set on a building entity.
pub fn on_building_selected(
    selected: Single<&building::Building, Added<building::SelectedBuilding>>,
    mut selected_building_ui: SelectedBuildingUi,
) {
    let beam_grid_pos = selected.grid_pos();
    let beam_pos = beam_grid_pos.world() + Vec3::Y * 0.6;

    selected_building_ui.show(selected.title(), selected.description(), beam_pos);
}

/// System called when a `[SelectedBuilding]` component is removed from a building entity.
pub fn on_building_deselected(
    _remove_event: On<Remove, building::SelectedBuilding>,
    mut selected_building_ui: SelectedBuildingUi,
) {
    selected_building_ui.hide();
}

/// Observer for when a construct gets selected.
pub fn on_construct_clicked(
    mut click: On<Pointer<Click>>,
    selected: crate::engine::systems::SelectedObjects,
    mut commands: Commands,
) {
    /* Deselect all selected buildings / constructs */
    selected.deselect_all(&mut commands);

    /* Set the newly selected entity as selected */
    commands.entity(click.entity).insert(construct::SelectedConstruct);

    click.propagate(false);
}

/// System called when a new `[SelectedConstruct]` component is set on a construct entity.
pub fn on_construct_selected(
    selected: Single<&construct::Construct, Added<construct::SelectedConstruct>>,
    mut selected_construct_ui: SelectedConstructUi,
) {
    let beam_grid_pos = selected.grid_pos();
    let beam_pos = beam_grid_pos.world() + Vec3::Y * 0.6;

    selected_construct_ui.show(selected.title(), selected.description(), beam_pos);
}

/// System called when a `[SelectedConstruct]` component is removed from a construct entity.
pub fn on_construct_deselected(
    _remove_event: On<Remove, construct::SelectedConstruct>,
    mut selected_construct_ui: SelectedConstructUi,
) {
    selected_construct_ui.hide();
}

/// System to update the position of the UI building panel info onto the selected building.
pub fn update_building_panel_position(
    camera: Single<(&Camera, &GlobalTransform)>,
    selected_building: Single<&GlobalTransform, With<building::SelectedBuilding>>,
    mut panel: Single<(&mut Node, &ComputedNode), With<ui::BuildingInfoPanel>>,
) {
    let (camera, camera_tf) = *camera;
    let (ui_node, ui_computed_node) = &mut *panel;

    update_ui_panel_pos(camera, camera_tf, &selected_building, ui_computed_node, ui_node, 1.2);
}

/// System to update the position of the UI construct panel info onto the selected construct.
pub fn update_construct_panel_position(
    camera: Single<(&Camera, &GlobalTransform)>,
    selected_construct: Single<&GlobalTransform, With<construct::SelectedConstruct>>,
    mut panel: Single<(&mut Node, &ComputedNode), With<ui::ConstructInfoPanel>>,
) {
    let (camera, camera_tf) = *camera;
    let (ui_node, ui_computed_node) = &mut *panel;

    update_ui_panel_pos(camera, camera_tf, &selected_construct, ui_computed_node, ui_node, 1.2);
}

/// Place the provided ui node on top of the world target, with respect to the provided camera.
fn update_ui_panel_pos(
    camera: &Camera,
    camera_tf: &GlobalTransform,
    world_target: &GlobalTransform,
    ui_computed_node: &ComputedNode,
    ui_node: &mut Node,
    height_offset: f32,
) {
    let anchor = world_target.translation() + Vec3::Y * height_offset;
    let screen_position = match camera.world_to_viewport(camera_tf, anchor) {
        Ok(position) => position,
        Err(_) => return,
    };

    let size = ui_computed_node.size() * ui_computed_node.inverse_scale_factor();
    ui_node.left = Val::Px(screen_position.x - size.x / 2.0);
    ui_node.top = Val::Px(screen_position.y - size.y);
}
