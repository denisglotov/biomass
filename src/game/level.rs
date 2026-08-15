use super::grid::{Edge, Grid};

#[derive(Debug, Clone)]
pub struct Level {
    pub title: String,
    pub description: String,
    pub rows: usize,
    pub cols: usize,
    pub walls_per_turn: usize,
    pub spread_steps: usize,
    pub max_threshold: usize,
    pub initial_biomass: Vec<(usize, usize)>,
    pub obstacles: Vec<(usize, usize)>,
    pub initial_walls: Vec<Edge>,
    /// (3-star max turns, 2-star max turns); above 2-star threshold gives 1 star
    pub star_thresholds: (usize, usize),
}

impl Level {
    pub fn create_initial_grid(&self) -> Grid {
        let mut grid = Grid::new(self.rows, self.cols);

        self.obstacles
            .iter()
            .for_each(|&(r, c)| grid.set_cell(r, c, super::grid::CellType::Obstacle));

        self.initial_biomass
            .iter()
            .for_each(|&(r, c)| grid.set_cell(r, c, super::grid::CellType::Biomass));

        self.initial_walls
            .iter()
            .for_each(|&edge| grid.set_edge(edge, super::grid::EdgeState::Wall));

        grid
    }
}

pub fn get_levels() -> Vec<Level> {
    vec![
        Level {
            title: "Level 1: Containment 101".to_string(),
            description: "Place walls on grid borders to trap and isolate biomass.".to_string(),
            rows: 5,
            cols: 5,
            walls_per_turn: 2,
            spread_steps: 1,
            max_threshold: 12,
            initial_biomass: vec![(2, 2)],
            obstacles: vec![],
            initial_walls: vec![],
            star_thresholds: (3, 5),
        },
        Level {
            title: "Level 2: Twin Spores".to_string(),
            description: "Multiple active outbreak points detected. Isolate both sectors."
                .to_string(),
            rows: 6,
            cols: 6,
            walls_per_turn: 2,
            spread_steps: 1,
            max_threshold: 18,
            initial_biomass: vec![(1, 1), (4, 4)],
            obstacles: vec![],
            initial_walls: vec![],
            star_thresholds: (3, 6),
        },
        Level {
            title: "Level 3: Pillar Defense".to_string(),
            description: "Facility structural pillars block biomass expansion and wall placement."
                .to_string(),
            rows: 6,
            cols: 6,
            walls_per_turn: 2,
            spread_steps: 1,
            max_threshold: 16,
            initial_biomass: vec![(2, 2)],
            obstacles: vec![(1, 1), (1, 4), (4, 1), (4, 4)],
            initial_walls: vec![],
            star_thresholds: (3, 6),
        },
        Level {
            title: "Level 4: Divided Sectors".to_string(),
            description: "Utilize existing facility security barriers to quickly enclose spores."
                .to_string(),
            rows: 7,
            cols: 7,
            walls_per_turn: 2,
            spread_steps: 1,
            max_threshold: 22,
            initial_biomass: vec![(1, 3), (5, 3)],
            obstacles: vec![],
            initial_walls: vec![
                Edge::Horizontal { r: 3, c: 2 },
                Edge::Horizontal { r: 3, c: 3 },
                Edge::Horizontal { r: 3, c: 4 },
            ],
            star_thresholds: (4, 7),
        },
        Level {
            title: "Level 5: Rapid Mutation".to_string(),
            description: "WARNING: High-speed mutation! Biomass expands 2 steps per turn."
                .to_string(),
            rows: 7,
            cols: 7,
            walls_per_turn: 3,
            spread_steps: 2,
            max_threshold: 25,
            initial_biomass: vec![(3, 3)],
            obstacles: vec![(2, 2), (4, 4)],
            initial_walls: vec![],
            star_thresholds: (4, 7),
        },
        Level {
            title: "Level 6: Central Rock Fortress".to_string(),
            description: "Four central containment pillars restrict open corridor pathways."
                .to_string(),
            rows: 8,
            cols: 8,
            walls_per_turn: 2,
            spread_steps: 1,
            max_threshold: 26,
            initial_biomass: vec![(1, 2), (6, 5)],
            obstacles: vec![(3, 3), (3, 4), (4, 3), (4, 4)],
            initial_walls: vec![],
            star_thresholds: (5, 8),
        },
        Level {
            title: "Level 7: Corridor Siege".to_string(),
            description: "Narrow corridor network. Seal off choke points before infection spreads."
                .to_string(),
            rows: 8,
            cols: 8,
            walls_per_turn: 3,
            spread_steps: 1,
            max_threshold: 28,
            initial_biomass: vec![(2, 2), (5, 5)],
            obstacles: vec![
                (0, 3),
                (1, 3),
                (6, 4),
                (7, 4),
                (3, 0),
                (3, 1),
                (4, 6),
                (4, 7),
            ],
            initial_walls: vec![],
            star_thresholds: (5, 8),
        },
        Level {
            title: "Level 8: Outbreak Zero".to_string(),
            description: "Triple spore outbreak across large grid layout.".to_string(),
            rows: 9,
            cols: 9,
            walls_per_turn: 3,
            spread_steps: 1,
            max_threshold: 32,
            initial_biomass: vec![(1, 4), (7, 1), (7, 7)],
            obstacles: vec![(4, 4)],
            initial_walls: vec![],
            star_thresholds: (5, 9),
        },
        Level {
            title: "Level 9: Bio-Lab Hazard".to_string(),
            description: "Multi-spore rapid outbreak with facility structural obstacles."
                .to_string(),
            rows: 9,
            cols: 9,
            walls_per_turn: 3,
            spread_steps: 2,
            max_threshold: 35,
            initial_biomass: vec![(2, 2), (6, 6)],
            obstacles: vec![(2, 4), (4, 2), (4, 6), (6, 4), (1, 1), (7, 7)],
            initial_walls: vec![],
            star_thresholds: (6, 9),
        },
        Level {
            title: "Level 10: Bio-Reactor Meltdown".to_string(),
            description:
                "FINAL PROTOCOL: 4 active spores in a 10x10 reactor core with tight capacity!"
                    .to_string(),
            rows: 10,
            cols: 10,
            walls_per_turn: 3,
            spread_steps: 2,
            max_threshold: 40,
            initial_biomass: vec![(2, 2), (2, 7), (7, 2), (7, 7)],
            obstacles: vec![
                (4, 4),
                (4, 5),
                (5, 4),
                (5, 5),
                (1, 4),
                (4, 1),
                (5, 8),
                (8, 5),
            ],
            initial_walls: vec![],
            star_thresholds: (6, 10),
        },
    ]
}
