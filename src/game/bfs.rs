use super::grid::{CellType, Grid};
use macroquad::rand::gen_range;
use std::collections::{HashSet, VecDeque};

/// Performs step-by-step biomass expansion up to `steps` distance.
/// Each active biomass cell produces 1 new cell per step (chosen randomly among adjacent free cells).
/// Returns a list of steps, where each step contains the coordinates `(r, c)` of newly infected cells.
pub fn expand_biomass_step_by_step(grid: &Grid, max_steps: usize) -> Vec<Vec<(usize, usize)>> {
    let mut steps_history = Vec::new();
    if max_steps == 0 {
        return steps_history;
    }

    let mut current_biomass: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.get_cell(r, c) == CellType::Biomass {
                current_biomass.insert((r, c));
            }
        }
    }

    let mut infected_this_turn: HashSet<(usize, usize)> = HashSet::new();

    for _step in 0..max_steps {
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
            let neighbors = [
                (r.wrapping_sub(1), c),
                (r + 1, c),
                (r, c.wrapping_sub(1)),
                (r, c + 1),
            ];

            let mut candidates = Vec::new();
            for &(nr, nc) in &neighbors {
                if grid.is_valid_cell(nr, nc)
                    && grid.get_cell(nr, nc) == CellType::Empty
                    && !current_biomass.contains(&(nr, nc))
                    && !infected_this_turn.contains(&(nr, nc))
                    && !grid.has_wall_between(r, c, nr, nc)
                {
                    candidates.push((nr, nc));
                }
            }

            if !candidates.is_empty() {
                let idx = gen_range(0, candidates.len());
                let (target_r, target_c) = candidates[idx];

                infected_this_turn.insert((target_r, target_c));
                step_newly_infected.push((target_r, target_c));
                frontier.push((target_r, target_c));
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

    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.get_cell(r, c) == CellType::Biomass && !visited.contains(&(r, c)) {
                // Collect all cells in this biomass component
                let mut component = Vec::new();
                let mut queue = VecDeque::new();

                queue.push_back((r, c));
                visited.insert((r, c));

                while let Some((cr, cc)) = queue.pop_front() {
                    component.push((cr, cc));

                    let neighbors = [
                        (cr.wrapping_sub(1), cc),
                        (cr + 1, cc),
                        (cr, cc.wrapping_sub(1)),
                        (cr, cc + 1),
                    ];

                    for &(nr, nc) in &neighbors {
                        if grid.is_valid_cell(nr, nc)
                            && grid.get_cell(nr, nc) == CellType::Biomass
                            && !visited.contains(&(nr, nc))
                            && !grid.has_wall_between(cr, cc, nr, nc)
                        {
                            visited.insert((nr, nc));
                            queue.push_back((nr, nc));
                        }
                    }
                }

                // Check if this biomass component has access to ANY empty cell
                let mut has_access_to_empty = false;
                let mut path_visited = HashSet::new();
                let mut path_queue = VecDeque::new();

                for &cell in &component {
                    path_queue.push_back(cell);
                    path_visited.insert(cell);
                }

                'search: while let Some((cr, cc)) = path_queue.pop_front() {
                    let neighbors = [
                        (cr.wrapping_sub(1), cc),
                        (cr + 1, cc),
                        (cr, cc.wrapping_sub(1)),
                        (cr, cc + 1),
                    ];

                    for &(nr, nc) in &neighbors {
                        if grid.is_valid_cell(nr, nc) && !grid.has_wall_between(cr, cc, nr, nc) {
                            let cell_type = grid.get_cell(nr, nc);
                            if cell_type == CellType::Empty {
                                has_access_to_empty = true;
                                break 'search;
                            } else if cell_type == CellType::Biomass
                                && !path_visited.contains(&(nr, nc))
                            {
                                path_visited.insert((nr, nc));
                                path_queue.push_back((nr, nc));
                            }
                        }
                    }
                }

                // If no empty cell is reachable, all cells in this component starve!
                if !has_access_to_empty {
                    starved_cells.extend(component);
                }
            }
        }
    }

    starved_cells
}
