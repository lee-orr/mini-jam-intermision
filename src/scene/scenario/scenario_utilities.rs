use super::*;

use crate::card::Targetable;

pub(crate) fn propose_valid_targets(
    actor: &Actor,
    targetable: &Targetable,
    positions: &[(Actor, ActorPosition)],
    map: &scenario_map::ScenarioMap,
    _resources: &ActorResources,
) -> Vec<(usize, usize)> {
    let my_position = positions
        .iter()
        .find_map(|(a, p)| if a == actor { Some(*p) } else { None });
    match targetable {
        Targetable::Path { max_distance } => {
            if let Some(my_position) = my_position {
                let positions = map
                    .tiles
                    .iter()
                    .filter_map(|t| {
                        if t.tile_type == scenario_map::TileType::Floor {
                            Some(t.pos)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                positions_within_n(&(my_position.0, my_position.1), &positions, *max_distance)
            } else {
                vec![]
            }
        }
    }
}

pub(crate) fn positions_within_n(
    position: &(usize, usize),
    positions: &[(usize, usize)],
    distance: usize
) -> Vec<(usize, usize)> {
    let mut checked = vec![];
    let mut to_check = vec![*position];
    let mut unchecked = positions
        .iter()
        .filter_map(|p| if p != position { Some(*p) } else { None })
        .collect::<Vec<_>>();

    for _ in 0..distance {
        let mut remove = vec![];
        let mut next_check = vec![];
        for pos in to_check.iter() {
            for (i, p) in unchecked.iter().enumerate() {
                if p.0.abs_diff(pos.0) <= 1 && p.1.abs_diff(pos.1) <= 1 {
                    remove.push(i);
                    if !checked.contains(p) {
                        next_check.push(*p);
                    }
                }
            }
        }
        unchecked = unchecked
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if remove.contains(&i) { None } else { Some(*v) })
            .collect();
        checked.append(&mut to_check);
        to_check.append(&mut next_check);
    }

    checked.append(&mut to_check);
    checked
}
