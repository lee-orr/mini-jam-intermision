use bevy::prelude::*;
use bevy_sequential_actions::ActionFinished;

use bevy_sequential_actions::StopReason;

use bevy_sequential_actions::ActionCommands;

use bevy_sequential_actions::Action;

use crate::scene::scenario::*;

pub struct AttackAction {
    pub(crate) target: ActorPosition,
    pub(crate) actor: Actor,
    pub(crate) duration: f32,
    pub(crate) damage: usize,
}

impl Action for AttackAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        // Run the wait system on the agent
        world
            .entity_mut(agent)
            .insert(Attack(self.actor, self.target, self.duration, None, self.damage));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, _reason: StopReason) {
        // Reattack the wait component from the agent
        let _wait = world.entity_mut(agent).remove::<Attack>();
    }
}

#[derive(Component)]
pub(crate) struct Attack(Actor, ActorPosition, f32, Option<f32>, usize);

pub(crate) fn attack_system(
    mut attack_q: Query<(&mut Attack, &mut ActionFinished)>,
    mut actors: Query<(&Actor, &ActorPosition, &mut Transform)>,
    mut events: EventWriter<AdjustActorEvent>,
    time: Res<Time>,
) {
    for (mut attack, mut finished) in attack_q.iter_mut() {
        if attack.3.is_none() {
            attack.3 = Some(time.elapsed_seconds());
        }
        if let Some(start) = attack.3 {
            let elapsed = time.elapsed_seconds() - start;
            let complete = elapsed >= attack.2;
            if complete  {
                finished.confirm_and_reset();
            }
            for (actor, pos, mut transform) in actors.iter_mut() {
                if actor == &attack.0 {
                    if complete {
                        transform.scale = Vec3::splat(1.);
                    } else {
                        transform.scale = Vec3::splat(1. + (elapsed / attack.2) * 0.5);
                    }
                }
                if complete {
                    if pos.clone() == attack.1 {
                        events.send(AdjustActorEvent::Damage(actor.clone(), attack.4));
                    }
                }
            }
        }
    }
}
