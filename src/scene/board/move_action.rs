use bevy::prelude::*;
use bevy_sequential_actions::ActionFinished;

use bevy_sequential_actions::StopReason;

use bevy_sequential_actions::ActionCommands;

use bevy_sequential_actions::Action;

use crate::scene::scenario::*;

pub struct MoveAction {
    pub(crate) position: ActorPosition,
    pub(crate) actor: Actor,
    pub(crate) speed: f32,
}

impl Action for MoveAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        // Run the wait system on the agent
        world
            .entity_mut(agent)
            .insert(Move(self.actor, self.position, self.speed));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, _reason: StopReason) {
        // Remove the wait component from the agent
        let _wait = world.entity_mut(agent).remove::<Move>();
    }
}

#[derive(Component)]
pub(crate) struct Move(Actor, ActorPosition, f32);

pub(crate) fn move_system(
    mut move_q: Query<(&mut Move, &mut ActionFinished)>,
    mut moveable: Query<(&Actor, &mut ActorPosition, &mut Transform)>,
    time: Res<Time>,
) {
    for (target, mut finished) in move_q.iter_mut() {
        for (actor, mut pos, mut transform) in moveable.iter_mut() {
            if actor == &target.0 {
                let current_position = transform.translation;
                let target_position =
                    Vec3::new(target.1 .0 as f32, current_position.y, target.1 .1 as f32);
                let delta = target_position - transform.translation;
                let distance_to_move = time.delta_seconds() * target.2;

                if delta.length_squared() <= distance_to_move {
                    transform.translation = target_position;
                    pos.0 = target.1 .0;
                    pos.1 = target.1 .1;
                    finished.confirm_and_reset();
                } else {
                    let move_vector = delta.normalize() * distance_to_move;
                    transform.translation += move_vector;
                }
            }
        }
    }
}
