use bevy::prelude::*;
use bevy_sequential_actions::ActionFinished;

use bevy_sequential_actions::StopReason;

use bevy_sequential_actions::ActionCommands;

use bevy_sequential_actions::Action;

use crate::scene::scenario::*;

pub struct StunAction {
    pub(crate) target: ActorPosition,
    pub(crate) actor: Actor,
    pub(crate) duration: f32,
    pub(crate) stun_duration: usize,
}

impl Action for StunAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        // Run the wait system on the agent
        world.entity_mut(agent).insert(Stun(
            self.actor,
            self.target,
            self.duration,
            None,
            self.stun_duration,
        ));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, _reason: StopReason) {
        // Restun the wait component from the agent
        let _wait = world.entity_mut(agent).remove::<Stun>();
    }
}

#[derive(Component)]
pub(crate) struct Stun(Actor, ActorPosition, f32, Option<f32>, usize);

pub(crate) fn stun_system(
    mut stun_q: Query<(&mut Stun, &mut ActionFinished)>,
    mut actors: Query<(&Actor, &ActorPosition, &mut Transform)>,
    mut events: EventWriter<AdjustActorEvent>,
    time: Res<Time>,
) {
    for (mut stun, mut finished) in stun_q.iter_mut() {
        if stun.3.is_none() {
            stun.3 = Some(time.elapsed_seconds());
        }
        if let Some(start) = stun.3 {
            let elapsed = time.elapsed_seconds() - start;
            let complete = elapsed >= stun.2;
            if complete {
                finished.confirm_and_reset();
            }
            for (actor, pos, mut transform) in actors.iter_mut() {
                if actor == &stun.0 {
                    if complete {
                        transform.scale = Vec3::splat(1.);
                    } else {
                        transform.scale = Vec3::splat(1. + (elapsed / stun.2) * 0.5);
                    }
                }
                if complete && *pos == stun.1 && actor != &stun.0 {
                    events.send(AdjustActorEvent::Stun(*actor, stun.4));
                }
            }
        }
    }
}
