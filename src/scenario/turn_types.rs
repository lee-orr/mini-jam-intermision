use bevy::prelude::*;

use crate::card::Card;

#[derive(Debug, Clone)]
pub struct TargetSelection {
    pub actor: Actor,
    pub card: Card,
    pub valid_targets: Vec<(usize, usize)>,
    pub num_targets_to_select: usize,
    pub action_id: usize,
}

#[derive(Debug, Clone)]
pub enum AnimateActionsEvents {
    Wait(f32),
    Continue(Actor),
    SelectTargets(TargetSelection),
    Move(Actor, ActorPosition),
    SetTurnProcess(CurrentTurnProcess),
}

#[derive(Debug, Clone, Resource)]
pub enum CurrentTurnProcess {
    None,
    Thinking(Actor),
    CardActionTriggered(Actor, Card, usize),
    CardTargetsSelected(Actor, Card, Vec<(usize, usize)>, usize),
    Done(Actor),
}

#[derive(Debug, Clone)]
pub struct CardPlayedEvent {
    pub actor: Actor,
    pub card: String,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Actor {
    Player,
    Enemy(usize),
}

impl PartialOrd for Actor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Actor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Actor::Player, Actor::Player) => 0usize.cmp(&0),
            (Actor::Player, Actor::Enemy(_)) => 0usize.cmp(&1),
            (Actor::Enemy(_), Actor::Player) => 1usize.cmp(&0),
            (Actor::Enemy(a), Actor::Enemy(b)) => (*a).cmp(b),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorPosition(pub usize, pub usize);
