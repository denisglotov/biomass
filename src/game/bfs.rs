use super::grid::{CellType, Grid};
use macroquad::rand::gen_range;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CloneEvent {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

/// Performs step-by-step biomass expansion up to `steps` distance, capped at `max_clones` total clones.
/// Each active biomass cell produces 1 new cell per step (chosen randomly among adjacent free cells).
/// If more cells are ready to clone than the remaining clone budget, only that number is randomly selected.
/// Returns a list of steps, where each step contains the clone events `CloneEvent { from, to }`.
pub fn expand_biomass_step_by_step(
    grid: &Grid,
    max_steps: usize,
    max_clones: usize,
) -> Vec<Vec<CloneEvent>> {
    let mut steps_history = Vec::new();
    if max_steps == 0 || max_clones == 0 {
        return steps_history;
    }

    let mut current_biomass: HashSet<(usize, usize)> = grid.active_biomass.clone();
    let mut infected_this_turn: HashSet<(usize, usize)> = HashSet::new();
    let mut total_clones = 0;

    for _step in 0..max_steps {
        if total_clones >= max_clones {
            break;
        }

        let mut step_newly_infected = Vec::new();
        let mut frontier: Vec<(usize, usize)> = Vec::new();

        let mut biomass_list: Vec<(usize, usize)> = current_biomass.iter().copied().collect();
        // Randomize biomass cell processing order so priority is fair
        if biomass_list.len() > 1 {
            for i in (1..biomass_list.len()).rev() {
                let j = gen_range(0, i + 1);
                biomass_list.swap(i, j);
            }
        }

        for &(r, c) in &biomass_list {
            if total_clones >= max_clones {
                break;
            }

            let candidates: Vec<_> = grid
                .valid_neighbors(r, c)
                .filter(|&(nr, nc)| {
                    grid.get_cell(nr, nc) == CellType::Empty
                        && !current_biomass.contains(&(nr, nc))
                        && !infected_this_turn.contains(&(nr, nc))
                        && !grid.has_wall_between(r, c, nr, nc)
                })
                .collect();

            if !candidates.is_empty() {
                let idx = gen_range(0, candidates.len());
                let (target_r, target_c) = candidates[idx];

                infected_this_turn.insert((target_r, target_c));
                step_newly_infected.push(CloneEvent {
                    from: (r, c),
                    to: (target_r, target_c),
                });
                frontier.push((target_r, target_c));
                total_clones += 1;
            }
        }

        if step_newly_infected.is_empty() {
            break; // No further expansion possible
        }

        for &(nr, nc) in &frontier {
            current_biomass.insert((nr, nc));
        }

        steps_history.push(step_newly_infected);
    }

    steps_history
}

/// Evaluates the Sealed Enclosure Rule.
/// A biomass component starves and dies off if it has NO reachable open path across passable edges
/// to ANY `CellType::Empty` cell on the grid.
/// Returns a list of cell coordinates `(r, c)` that deactivate (die off).
pub fn evaluate_sealed_enclosure_dieoff(grid: &Grid) -> Vec<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut starved_cells = Vec::new();

    for &(r, c) in &grid.active_biomass {
        if !visited.contains(&(r, c)) {
            let mut component = Vec::new();
            let mut queue = VecDeque::new();
            let mut has_access_to_empty = false;

            queue.push_back((r, c));
            visited.insert((r, c));

            while let Some((cr, cc)) = queue.pop_front() {
                component.push((cr, cc));

                for (nr, nc) in grid.valid_neighbors(cr, cc) {
                    if grid.has_wall_between(cr, cc, nr, nc) {
                        continue;
                    }

                    match grid.get_cell(nr, nc) {
                        CellType::Empty => {
                            has_access_to_empty = true;
                        }
                        CellType::Biomass => {
                            if !visited.contains(&(nr, nc)) {
                                visited.insert((nr, nc));
                                queue.push_back((nr, nc));
                            }
                        }
                        CellType::Obstacle => {}
                    }
                }
            }

            if !has_access_to_empty {
                starved_cells.extend(component);
            }
        }
    }

    starved_cells
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::grid::{CellType, Edge, EdgeState, Grid};
    use std::collections::HashSet;

    #[test]
    fn no_biomass_no_dieoff() {
        let grid = Grid::new(3, 3);
        assert!(evaluate_sealed_enclosure_dieoff(&grid).is_empty());
    }

    #[test]
    fn unsealed_biomass_survives() {
        // Single biomass cell surrounded by empty — should not die.
        let mut grid = Grid::new(3, 3);
        grid.set_cell(1, 1, CellType::Biomass);
        assert!(evaluate_sealed_enclosure_dieoff(&grid).is_empty());
    }

    #[test]
    fn fully_walled_biomass_dies() {
        // Biomass at (1,1) in a 3x3 grid, walled off on all 4 sides.
        //
        //   +---+---+---+
        //   |   |   |   |
        //   +---+===+---+   <- horizontal wall at r=1, c=1
        //   |   ‖ B ‖   |
        //   +---+===+---+   <- horizontal wall at r=2, c=1
        //   |   |   |   |
        //   +---+---+---+
        let mut grid = Grid::new(3, 3);
        grid.set_cell(1, 1, CellType::Biomass);
        grid.set_edge(Edge::Horizontal { r: 1, c: 1 }, EdgeState::Wall);
        grid.set_edge(Edge::Horizontal { r: 2, c: 1 }, EdgeState::Wall);
        grid.set_edge(Edge::Vertical { r: 1, c: 1 }, EdgeState::Wall);
        grid.set_edge(Edge::Vertical { r: 1, c: 2 }, EdgeState::Wall);

        assert_eq!(evaluate_sealed_enclosure_dieoff(&grid), vec![(1, 1)]);
    }

    #[test]
    fn walled_box_with_gap_survives() {
        // Same as above but leave one wall open — biomass can reach empty.
        let mut grid = Grid::new(3, 3);
        grid.set_cell(1, 1, CellType::Biomass);
        grid.set_edge(Edge::Horizontal { r: 1, c: 1 }, EdgeState::Wall);
        grid.set_edge(Edge::Horizontal { r: 2, c: 1 }, EdgeState::Wall);
        grid.set_edge(Edge::Vertical { r: 1, c: 1 }, EdgeState::Wall);
        // Vertical r=1, c=2 left open

        assert!(evaluate_sealed_enclosure_dieoff(&grid).is_empty());
    }

    #[test]
    fn obstacle_assisted_seal() {
        // Biomass at (0,0) in a 2x2 grid, obstacle at (0,1) and (1,0).
        // Walls between (0,0)-(1,1) aren't needed since they aren't adjacent.
        // The only non-obstacle neighbor is (1,1) diagonally — not reachable.
        // Outer boundary edges are already impassable (grid boundary).
        // Obstacle neighbors block expansion, so biomass is sealed.
        let mut grid = Grid::new(2, 2);
        grid.set_cell(0, 0, CellType::Biomass);
        grid.set_cell(0, 1, CellType::Obstacle);
        grid.set_cell(1, 0, CellType::Obstacle);
        // (1,1) is empty but not adjacent to (0,0)

        assert_eq!(evaluate_sealed_enclosure_dieoff(&grid), vec![(0, 0)]);
    }

    #[test]
    fn two_components_one_sealed() {
        // 1x4 grid: [Biomass | Wall | Biomass | Empty]
        // Left biomass is walled off, right biomass has access to empty.
        let mut grid = Grid::new(1, 4);
        grid.set_cell(0, 0, CellType::Biomass);
        grid.set_cell(0, 2, CellType::Biomass);
        // Wall between (0,0) and (0,1)
        grid.set_edge(Edge::Vertical { r: 0, c: 1 }, EdgeState::Wall);
        // (0,1) is empty but walled off from (0,0)
        // (0,3) is empty and reachable from (0,2)

        let result = evaluate_sealed_enclosure_dieoff(&grid);
        assert_eq!(result, vec![(0, 0)]);
    }

    #[test]
    fn connected_component_partially_borders_empty() {
        // 1x3 grid: [Biomass, Biomass, Empty] — no walls.
        // Both biomass cells form one component with access to empty.
        let mut grid = Grid::new(1, 3);
        grid.set_cell(0, 0, CellType::Biomass);
        grid.set_cell(0, 1, CellType::Biomass);

        assert!(evaluate_sealed_enclosure_dieoff(&grid).is_empty());
    }

    #[test]
    fn biomass_fills_entire_grid() {
        // All cells are biomass — no empty cells reachable, all die.
        let mut grid = Grid::new(2, 2);
        grid.set_cell(0, 0, CellType::Biomass);
        grid.set_cell(0, 1, CellType::Biomass);
        grid.set_cell(1, 0, CellType::Biomass);
        grid.set_cell(1, 1, CellType::Biomass);

        let result: HashSet<_> = evaluate_sealed_enclosure_dieoff(&grid)
            .into_iter()
            .collect();
        let expected: HashSet<_> = [(0, 0), (0, 1), (1, 0), (1, 1)].into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn sealed_multi_cell_component_dies() {
        // 3x3 grid, 2x2 biomass block at center-ish, fully walled off.
        //   +---+---+---+
        //   |   |   |   |
        //   +---+===+===+
        //   |   ‖ B | B ‖
        //   +---+---+---+
        //   |   ‖ B | B ‖
        //   +---+===+===+
        let mut grid = Grid::new(3, 3);
        grid.set_cell(1, 1, CellType::Biomass);
        grid.set_cell(1, 2, CellType::Biomass);
        grid.set_cell(2, 1, CellType::Biomass);
        grid.set_cell(2, 2, CellType::Biomass);
        // Top wall
        grid.set_edge(Edge::Horizontal { r: 1, c: 1 }, EdgeState::Wall);
        grid.set_edge(Edge::Horizontal { r: 1, c: 2 }, EdgeState::Wall);
        // Bottom wall (r=3 is the boundary below row 2)
        grid.set_edge(Edge::Horizontal { r: 3, c: 1 }, EdgeState::Wall);
        grid.set_edge(Edge::Horizontal { r: 3, c: 2 }, EdgeState::Wall);
        // Left wall
        grid.set_edge(Edge::Vertical { r: 1, c: 1 }, EdgeState::Wall);
        grid.set_edge(Edge::Vertical { r: 2, c: 1 }, EdgeState::Wall);
        // Right wall (c=3 is the boundary right of col 2)
        grid.set_edge(Edge::Vertical { r: 1, c: 3 }, EdgeState::Wall);
        grid.set_edge(Edge::Vertical { r: 2, c: 3 }, EdgeState::Wall);

        let result: HashSet<_> = evaluate_sealed_enclosure_dieoff(&grid)
            .into_iter()
            .collect();
        let expected: HashSet<_> = [(1, 1), (1, 2), (2, 1), (2, 2)].into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn expand_biomass_records_clone_events() {
        let mut grid = Grid::new(1, 3);
        grid.set_cell(0, 0, CellType::Biomass);
        let steps = expand_biomass_step_by_step(&grid, 1, 4);
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].len(), 1);
        assert_eq!(
            steps[0][0],
            CloneEvent {
                from: (0, 0),
                to: (0, 1)
            }
        );
    }

    #[test]
    fn expand_biomass_respects_max_clones_limit() {
        // 5 isolated biomass cells with empty space to the right
        let mut grid = Grid::new(5, 3);
        for r in 0..5 {
            grid.set_cell(r, 0, CellType::Biomass);
        }
        // max_clones = 2 even though 5 cells are ready to clone
        let steps = expand_biomass_step_by_step(&grid, 1, 2);
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].len(), 2);
        // All cloned events must be valid
        for event in &steps[0] {
            assert_eq!(grid.get_cell(event.from.0, event.from.1), CellType::Biomass);
            assert_eq!(grid.get_cell(event.to.0, event.to.1), CellType::Empty);
        }
    }

    #[test]
    fn expand_biomass_zero_max_clones() {
        let mut grid = Grid::new(3, 3);
        grid.set_cell(1, 1, CellType::Biomass);
        let steps = expand_biomass_step_by_step(&grid, 1, 0);
        assert!(steps.is_empty());
    }

    #[test]
    fn expand_biomass_multi_step_capped() {
        // Grid with a single biomass cell and 2 steps, but max_clones = 2
        let mut grid = Grid::new(5, 5);
        grid.set_cell(2, 2, CellType::Biomass);
        let steps = expand_biomass_step_by_step(&grid, 2, 2);
        let total_clones: usize = steps.iter().map(|step| step.len()).sum();
        assert!(total_clones <= 2);
    }
}
