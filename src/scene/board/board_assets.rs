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
    pub(crate) dead_character_mat: Handle<StandardMaterial>,
    pub(crate) goal_mat: Handle<StandardMaterial>,
    pub(crate) goal_succeeded_mat: Handle<StandardMaterial>,
    pub(crate) start_point_mat: Handle<StandardMaterial>,
    pub(crate) selector: Handle<Mesh>,
    pub(crate) selector_mat: Handle<StandardMaterial>,
    pub(crate) selector_active: Handle<StandardMaterial>,
    pub(crate) selector_hover: Handle<StandardMaterial>,
}
pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile = meshes.add(shape::Box::new(1., 0.2, 1.).into());
    let wall = meshes.add(shape::Box::new(1., 2., 1.).into());
    let obstacle = meshes.add(shape::Box::new(1., 0.6, 1.).into());
    let monster = meshes.add(shape::Box::new(0.3, 1.8, 0.2).into());
    let player = meshes.add(shape::Capsule::default().into());

    let tile_mat = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..Default::default()
    });
    let goal_mat = materials.add(StandardMaterial {
        base_color: Color::GOLD,
        ..Default::default()
    });
    let goal_succeeded_mat = materials.add(StandardMaterial {
        base_color: Color::DARK_GRAY,
        ..Default::default()
    });
    let start_point_mat = materials.add(StandardMaterial {
        base_color: Color::GREEN,
        ..Default::default()
    });
    let monster_mat = materials.add(StandardMaterial {
        base_color: Color::PURPLE,
        ..Default::default()
    });
    let player_mat = materials.add(StandardMaterial {
        base_color: Color::BLUE,
        ..Default::default()
    });
    let dead_character_mat = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..Default::default()
    });
    let selector_mat = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 0.9,
            green: 0.8,
            blue: 0.2,
            alpha: 0.3,
        },
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let selector_hover = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 0.7,
            green: 0.7,
            blue: 0.2,
            alpha: 0.5,
        },
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let selector_active = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 0.2,
            green: 0.8,
            blue: 0.5,
            alpha: 0.5,
        },
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    commands.insert_resource(BoardAssets {
        tile,
        selector: wall.clone(),
        wall,
        obstacle,
        tile_mat,
        goal_mat,
        start_point_mat,
        monster,
        player,
        monster_mat,
        player_mat,
        selector_mat,
        selector_active,
        selector_hover,
        goal_succeeded_mat,
        dead_character_mat,
    });
}
