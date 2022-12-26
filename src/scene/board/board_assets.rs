use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Default, AssetCollection, Resource)]
pub(crate) struct BoardAssets {
    #[asset(path = "models.gltf#Mesh0/Primitive0")]
    pub(crate) tile: Handle<Mesh>,
    #[asset(path = "models.gltf#Mesh1/Primitive0")]
    pub(crate) wall: Handle<Mesh>,
    #[asset(path = "models.gltf#Mesh2/Primitive0")]
    pub(crate) obstacle: Handle<Mesh>,
    #[asset(path = "models.gltf#Mesh6/Primitive0")]
    pub(crate) monster: Handle<Mesh>,
    #[asset(path = "models.gltf#Mesh3/Primitive0")]
    pub(crate) player: Handle<Mesh>,
    #[asset(path = "models.gltf#Material0")]
    pub(crate) tile_mat: Handle<StandardMaterial>,
    #[asset(path = "models.gltf#Material1")]
    pub(crate) wall_mat: Handle<StandardMaterial>,
    #[asset(path = "models.gltf#Material2")]
    pub(crate) obstacle_mat: Handle<StandardMaterial>,
    #[asset(path = "models.gltf#Material7")]
    pub(crate) monster_mat: Handle<StandardMaterial>,
    #[asset(path = "models.gltf#Material4")]
    pub(crate) player_mat: Handle<StandardMaterial>,
    #[asset(path = "models.gltf#Material9")]
    pub(crate) dead_character_mat: Handle<StandardMaterial>,
    #[asset(path = "models.gltf#Material5")]
    pub(crate) goal_mat: Handle<StandardMaterial>,
    #[asset(path = "models.gltf#Material6")]
    pub(crate) goal_succeeded_mat: Handle<StandardMaterial>,
    #[asset(path = "models.gltf#Mesh7/Primitive0")]
    pub(crate) selector: Handle<Mesh>,
    #[asset(path = "models.gltf#Material8")]
    pub(crate) selector_mat: Handle<StandardMaterial>,
    #[asset(path = "models.gltf#Material11")]
    pub(crate) selector_active: Handle<StandardMaterial>,
    #[asset(path = "models.gltf#Material10")]
    pub(crate) selector_hover: Handle<StandardMaterial>,
}
