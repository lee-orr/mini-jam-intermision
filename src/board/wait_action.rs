use bevy::prelude::*;
use bevy_sequential_actions::ActionFinished;

use bevy_sequential_actions::StopReason;

use bevy_sequential_actions::ActionCommands;

use bevy_sequential_actions::Action;

pub struct WaitAction {
    pub(crate) duration: f32,
    pub(crate) current: Option<f32>,
}

impl Action for WaitAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        // Take current duration (if paused), or use full duration
        let duration = self.current.take().unwrap_or(self.duration);

        // Run the wait system on the agent
        world.entity_mut(agent).insert(Wait(duration));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, reason: StopReason) {
        // Remove the wait component from the agent
        let wait = world.entity_mut(agent).remove::<Wait>();

        // Store current duration when paused
        if let StopReason::Paused = reason {
            self.current = Some(wait.unwrap().0);
        }
    }
}

#[derive(Component)]
pub(crate) struct Wait(f32);

pub(crate) fn wait_system(mut wait_q: Query<(&mut Wait, &mut ActionFinished)>, time: Res<Time>) {
    for (mut wait, mut finished) in wait_q.iter_mut() {
        wait.0 -= time.delta_seconds();

        // Confirm finished state every frame
        if wait.0 <= 0.0 {
            finished.confirm_and_reset();
        }
    }
}
