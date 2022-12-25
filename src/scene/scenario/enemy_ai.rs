use crate::card::Cards;
use bevy::prelude::*;

use super::*;

use bevy_turborand::GlobalRng;

use bevy_turborand::DelegatedRng;

pub fn select_target<T: DelegatedRng>(
    rng: &mut T,
    selection: &TargetSelection,
) -> Vec<(usize, usize)> {
    let mut selected = Vec::with_capacity(selection.num_targets_to_select);
    let valid_target_len = selection.valid_targets.len();
    let range = 0..valid_target_len;

    while selected.len() < selection.num_targets_to_select && selected.len() < valid_target_len {
        let mut next = rng.usize(range.clone());
        while selected.contains(&next) {
            next = rng.usize(range.clone());
        }
        selected.push(next);
    }

    selected
        .iter()
        .filter_map(|i| selection.valid_targets.get(*i).copied())
        .collect()
}

pub(crate) fn choose_enemy_card(
    mut commands: Commands,
    mut events: EventWriter<CardPlayedEvent>,
    mut global_rng: ResMut<GlobalRng>,
    current_turn_process: Option<Res<CurrentTurnProcess>>,
    resources: Option<Res<ActorResources>>,
    _cards: Res<Cards>,
) {
    info!("Choosing enemy card...");
    if let (Some(process), Some(resources)) = (current_turn_process, resources) {
        info!("Process can continue");
        match *process {
            CurrentTurnProcess::Thinking(actor) => {
                if let Some(res) = resources.resources.get(&actor) {
                    let hand = &res.hand;
                    let range = 0..hand.len();
                    let selected = global_rng.usize(range);
                    if let Some(selected) = hand.get(selected) {
                        info!("Playing a card {:?}", selected);
                        events.send(CardPlayedEvent {
                            actor,
                            card: selected.clone(),
                        });
                        return;
                    }
                }
                info!("Couldn't play anything, skipping turn");
                commands.insert_resource(CurrentTurnProcess::Done(actor));
            }
            _ => {
                info!("Not thinking - why?");
            }
        }
    }
}
