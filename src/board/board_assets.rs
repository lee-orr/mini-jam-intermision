
    use bevy::prelude::*;
    #[derive(Default, Resource)]
    pub(crate) struct BoardAssets {
        pub(crate) tile: Handle<Mesh>,
        pub(crate) wall: Handle<Mesh>,
        pub(crate) obstacle: Handle<Mesh>,
        pub(crate) monster: Handle<Mesh>,
        pub(crate) player: Handle<Mesh>,
        pub(crate) tile_mat: Handle<StandardMaterial>,
        pub(crate) monster_mat: Handle<StandardMaterial>,
        pub(crate) player_mat: Handle<StandardMaterial>,
        pub(crate) goal_mat: Handle<StandardMaterial>,
        pub(crate) start_point_mat: Handle<StandardMaterial>,
        pub(crate) selector: Handle<Mesh>,
        pub(crate) selector_mat: Handle<StandardMaterial>,
        pub(crate) selector_active: Handle<StandardMaterial>,
        pub(crate) selector_hover: Handle<StandardMaterial>,
    }
