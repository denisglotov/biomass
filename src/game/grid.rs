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
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            cells: vec![CellType::Empty; rows * cols],
            h_walls: vec![EdgeState::Passable; (rows + 1) * cols],
            v_walls: vec![EdgeState::Passable; rows * (cols + 1)],
        }
    }

    #[inline]
    pub fn is_valid_cell(&self, r: usize, c: usize) -> bool {
        r < self.rows && c < self.cols
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
        }
    }

    pub fn count_biomass(&self) -> usize {
        self.cells
            .iter()
            .filter(|&&c| c == CellType::Biomass)
            .count()
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
            let min_r = r1.max(r2);
            self.get_edge(Edge::Horizontal { r: min_r, c: c1 }) == EdgeState::Wall
        } else {
            let min_c = c1.max(c2);
            self.get_edge(Edge::Vertical { r: r1, c: min_c }) == EdgeState::Wall
        }
    }

    /// Can a wall be placed on this edge? (Must be Passable and not outer boundary or bordering obstacle on both sides)
    pub fn can_place_wall(&self, edge: Edge) -> bool {
        if self.get_edge(edge) == EdgeState::Wall {
            return false;
        }

        match edge {
            Edge::Horizontal { r, c } => {
                if r == 0 || r == self.rows {
                    return false; // Outer boundary
                }
                // Cannot place wall if both adjacent cells are Obstacle
                let cell_above = self.get_cell(r - 1, c);
                let cell_below = self.get_cell(r, c);
                cell_above != CellType::Obstacle || cell_below != CellType::Obstacle
            }
            Edge::Vertical { r, c } => {
                if c == 0 || c == self.cols {
                    return false; // Outer boundary
                }
                let cell_left = self.get_cell(r, c - 1);
                let cell_right = self.get_cell(r, c);
                cell_left != CellType::Obstacle || cell_right != CellType::Obstacle
            }
        }
    }

    /// Check if there are any remaining legal wall placements on open passable internal edges
    pub fn has_any_legal_wall_placement(&self) -> bool {
        for r in 1..self.rows {
            for c in 0..self.cols {
                if self.can_place_wall(Edge::Horizontal { r, c }) {
                    return true;
                }
            }
        }
        for r in 0..self.rows {
            for c in 1..self.cols {
                if self.can_place_wall(Edge::Vertical { r, c }) {
                    return true;
                }
            }
        }
        false
    }
}
