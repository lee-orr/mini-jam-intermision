use bevy::prelude::*;
use bevy_sequential_actions::ActionFinished;

use bevy_sequential_actions::StopReason;

use bevy_sequential_actions::ActionCommands;

use bevy_sequential_actions::Action;

use crate::scenario::*;

pub struct SetTurnProcessAction(pub CurrentTurnProcess);

impl Action for SetTurnProcessAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        // Run the wait system on the agent
        world
            .entity_mut(agent)
            .insert(SetTurnProcess(self.0.clone()));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, _reason: StopReason) {
        // Remove the wait component from the agent
        let _cont = world.entity_mut(agent).remove::<SetTurnProcess>();
    }
}

#[derive(Component)]
pub(crate) struct SetTurnProcess(CurrentTurnProcess);

pub(crate) fn set_turn_process_system(
    mut wait_q: Query<(&mut SetTurnProcess, &mut ActionFinished)>,
    mut commands: Commands,
) {
    for (cont, mut finished) in wait_q.iter_mut() {
        finished.confirm_and_reset();
        commands.insert_resource(cont.0.clone());
    }
}
