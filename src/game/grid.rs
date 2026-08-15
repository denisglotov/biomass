#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Biomass,
    Obstacle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeState {
    Passable,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Edge {
    Horizontal { r: usize, c: usize }, // Wall between cell (r-1, c) and (r, c), for r in 0..=rows, c in 0..cols
    Vertical { r: usize, c: usize }, // Wall between cell (r, c-1) and (r, c), for r in 0..rows, c in 0..=cols
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<CellType>,
    pub h_walls: Vec<EdgeState>,
    pub v_walls: Vec<EdgeState>,
    pub active_biomass: std::collections::HashSet<(usize, usize)>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            cells: vec![CellType::Empty; rows * cols],
            h_walls: vec![EdgeState::Passable; (rows + 1) * cols],
            v_walls: vec![EdgeState::Passable; rows * (cols + 1)],
            active_biomass: std::collections::HashSet::new(),
        }
    }

    #[inline]
    pub fn is_valid_cell(&self, r: usize, c: usize) -> bool {
        r < self.rows && c < self.cols
    }

    pub fn valid_neighbors(&self, r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        [
            (r.wrapping_sub(1), c),
            (r + 1, c),
            (r, c.wrapping_sub(1)),
            (r, c + 1),
        ]
        .into_iter()
        .filter(move |&(nr, nc)| self.is_valid_cell(nr, nc))
    }

    #[inline]
    pub fn cell_idx(&self, r: usize, c: usize) -> usize {
        r * self.cols + c
    }

    pub fn get_cell(&self, r: usize, c: usize) -> CellType {
        if self.is_valid_cell(r, c) {
            self.cells[self.cell_idx(r, c)]
        } else {
            CellType::Obstacle
        }
    }

    pub fn set_cell(&mut self, r: usize, c: usize, cell_type: CellType) {
        if self.is_valid_cell(r, c) {
            let idx = self.cell_idx(r, c);
            self.cells[idx] = cell_type;

            if cell_type == CellType::Biomass {
                self.active_biomass.insert((r, c));
            } else {
                self.active_biomass.remove(&(r, c));
            }
        }
    }

    pub fn count_biomass(&self) -> usize {
        self.active_biomass.len()
    }

    pub fn get_edge(&self, edge: Edge) -> EdgeState {
        match edge {
            Edge::Horizontal { r, c } => {
                if r <= self.rows && c < self.cols {
                    self.h_walls[r * self.cols + c]
                } else {
                    EdgeState::Wall
                }
            }
            Edge::Vertical { r, c } => {
                if r < self.rows && c <= self.cols {
                    self.v_walls[r * (self.cols + 1) + c]
                } else {
                    EdgeState::Wall
                }
            }
        }
    }

    pub fn set_edge(&mut self, edge: Edge, state: EdgeState) {
        match edge {
            Edge::Horizontal { r, c } => {
                if r <= self.rows && c < self.cols {
                    self.h_walls[r * self.cols + c] = state;
                }
            }
            Edge::Vertical { r, c } => {
                if r < self.rows && c <= self.cols {
                    self.v_walls[r * (self.cols + 1) + c] = state;
                }
            }
        }
    }

    /// Checks if a wall exists between two adjacent cells (r1, c1) and (r2, c2)
    pub fn has_wall_between(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> bool {
        if (r1 as isize - r2 as isize).abs() + (c1 as isize - c2 as isize).abs() != 1 {
            return true;
        }

        if r1 != r2 {
            let wall_r = r1.max(r2);
            self.get_edge(Edge::Horizontal { r: wall_r, c: c1 }) == EdgeState::Wall
        } else {
            let wall_c = c1.max(c2);
            self.get_edge(Edge::Vertical { r: r1, c: wall_c }) == EdgeState::Wall
        }
    }

    /// Can a wall be placed on this edge? (Must be Passable and not outer boundary or bordering obstacle on both sides)
    pub fn can_place_wall(&self, edge: Edge) -> bool {
        if self.get_edge(edge) == EdgeState::Wall {
            return false;
        }

        match edge {
            Edge::Horizontal { r, c } => {
                (r > 0 && r < self.rows)
                    && (self.get_cell(r - 1, c) != CellType::Obstacle
                        || self.get_cell(r, c) != CellType::Obstacle)
            }
            Edge::Vertical { r, c } => {
                (c > 0 && c < self.cols)
                    && (self.get_cell(r, c - 1) != CellType::Obstacle
                        || self.get_cell(r, c) != CellType::Obstacle)
            }
        }
    }

    /// Check if there are any remaining legal wall placements on open passable internal edges
    pub fn has_any_legal_wall_placement(&self) -> bool {
        (1..self.rows)
            .any(|r| (0..self.cols).any(|c| self.can_place_wall(Edge::Horizontal { r, c })))
            || (0..self.rows)
                .any(|r| (1..self.cols).any(|c| self.can_place_wall(Edge::Vertical { r, c })))
    }
}
