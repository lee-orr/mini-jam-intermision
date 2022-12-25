
use bevy_sequential_actions::ActionFinished;

use bevy_mod_picking::PickingEvent;

use bevy_mod_picking::NoDeselect;

use bevy_mod_picking::Highlighting;

use bevy_mod_picking::PickableBundle;

use bevy::prelude::*;

use super::board_assets::BoardAssets;

use bevy_sequential_actions::StopReason;

use bevy_sequential_actions::ActionCommands;

use bevy_sequential_actions::Action;

use crate::scene::scenario::*;

pub(crate) struct SelectTargetsAction(pub TargetSelection);

impl Action for SelectTargetsAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        world
            .entity_mut(agent)
            .insert(SelectTargets(self.0.clone()));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, _reason: StopReason) {
        let _cont = world.entity_mut(agent).remove::<SelectTargets>();
    }
}

#[derive(Component)]
pub(crate) struct SelectTargets(TargetSelection);

#[derive(Component)]
pub(crate) struct Selectable(usize, usize);

#[derive(Component)]
pub(crate) struct Selected;

pub(crate) fn setup_selectable(
    mut commands: Commands,
    board: Query<(Entity, &SelectTargets), Changed<SelectTargets>>,
    assets: Res<BoardAssets>,
) {
    if let Ok((board, select_targets)) = board.get_single() {
        bevy::log::info!("Displaying Selected Targets");
        commands.entity(board).add_children(|parent| {
            for pos in select_targets.0.valid_targets.iter() {
                parent.spawn((
                    Selectable(pos.0, pos.1),
                    PickableBundle::default(),
                    Highlighting {
                        initial: assets.selector_mat.clone(),
                        hovered: Some(assets.selector_hover.clone()),
                        pressed: Some(assets.selector_active.clone()),
                        selected: Some(assets.selector_active.clone()),
                    },
                    NoDeselect,
                    PbrBundle {
                        mesh: assets.selector.clone(),
                        material: assets.selector_mat.clone(),
                        transform: Transform::from_xyz(pos.0 as f32, 0., pos.1 as f32),
                        ..Default::default()
                    },
                ));
            }
        });
    }
}

pub(crate) fn process_selection_events(
    mut events: EventReader<PickingEvent>,
    mut commands: Commands,
    assets: Res<BoardAssets>,
    mut selectables: Query<
        (
            Entity,
            Option<&Selected>,
            &mut Highlighting<StandardMaterial>,
        ),
        With<Selectable>,
    >,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => {
                info!("Clicked...");
                if let Ok((entity, selected, mut highlighting)) = selectables.get_mut(*e) {
                    if selected.is_none() {
                        info!("Selected");
                        commands.entity(entity).insert(Selected);
                        highlighting.initial = assets.selector_active.clone();
                    } else {
                        info!("Deselected");
                        commands.entity(entity).remove::<Selected>();
                        highlighting.initial = assets.selector_mat.clone();
                    }
                }
            }
        }
    }
}

pub(crate) fn set_selection(
    mut commands: Commands,
    mut board: Query<(Entity, &SelectTargets, &mut ActionFinished)>,
    selected: Query<(Entity, &Selectable), With<Selected>>,
    selectable: Query<Entity, With<Selectable>>,
) {
    if let Ok((_board, select_targets, mut action_finished)) = board.get_single_mut() {
        let select = &select_targets.0;
        let count = selected.iter().count();
        if count >= select.num_targets_to_select || count >= select.valid_targets.len() {
            info!("Got all selections");
            let selected = selected.iter().map(|(_, s)| (s.0, s.1)).collect();
            let process = CurrentTurnProcess::CardTargetsSelected(
                select.actor,
                select.card.clone(),
                selected,
                select.action_id,
            );
            info!("Clearing Selectables");
            for e in selectable.iter() {
                commands.entity(e).despawn_recursive();
            }
            commands.insert_resource(process);
            action_finished.confirm_and_reset();
        }
    }
}
