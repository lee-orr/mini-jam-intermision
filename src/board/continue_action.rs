use crate::scene::SceneState;

use bevy_sequential_actions::ActionFinished;

use bevy_sequential_actions::StopReason;

use bevy_sequential_actions::ActionCommands;

use bevy::prelude::*;
use bevy_sequential_actions::Action;

use crate::scenario::Actor;

pub struct ContinueAction(pub Actor);

impl Action for ContinueAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        // Run the wait system on the agent
        world.entity_mut(agent).insert(Continue(self.0));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, _reason: StopReason) {
        // Remove the wait component from the agent
        let _cont = world.entity_mut(agent).remove::<Continue>();
    }
}

#[derive(Component)]
pub(crate) struct Continue(Actor);

pub(crate) fn continue_system(
    mut wait_q: Query<(&mut Continue, &mut ActionFinished)>,
    mut scene_state: ResMut<State<SceneState>>,
) {
    for (cont, mut finished) in wait_q.iter_mut() {
        finished.confirm_and_reset();
        info!("Continuing... {:?}", cont.0);
        if cont.0 == Actor::Player {
            let _ = scene_state.set(SceneState::PlayerTurn);
        } else {
            let _ = scene_state.set(SceneState::EnemyTurn);
        }
    }
}
