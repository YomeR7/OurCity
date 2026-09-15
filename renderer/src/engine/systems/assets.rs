use bevy::prelude::*;

/// System to load all assets into the ECS
pub fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    /* Load all assets */

    /* House and house construct */
    let house = assets.load(GltfAssetLabel::Scene(0).from_asset("house.glb"));
    let house_construct = assets.load(GltfAssetLabel::Scene(0).from_asset("house_construct.glb"));

    let assets_resources = crate::engine::resources::BuildingAssets { house, house_construct };

    commands.insert_resource(assets_resources);
}
