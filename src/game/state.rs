use super::bfs::{evaluate_sealed_enclosure_dieoff, expand_biomass_step_by_step, CloneEvent};
use super::grid::{CellType, Edge, EdgeState, Grid};
use super::level::{get_levels, Level};
use super::storage::{load_last_level_reached, save_last_level_reached};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    PlayerTurn,
    BiomassExpansion,
    IsolationCheck,
    Victory,
    Defeat,
}

pub struct GameState {
    pub levels: Vec<Level>,
    pub current_level_idx: usize,
    pub level: Level,
    pub grid: Grid,
    pub turn_number: usize,
    pub walls_left: usize,
    pub phase: GamePhase,
    pub placed_walls_this_turn: Vec<Edge>,

    // Animation state
    pub anim_timer: f32,
    pub expansion_steps: Vec<Vec<CloneEvent>>,
    pub current_anim_step: usize,
    pub dying_biomass: Vec<(usize, usize)>,
    pub newly_cloned_this_step: Vec<CloneEvent>,
    pub newly_starved_this_step: Vec<(usize, usize)>,

    // End-of-level stats
    pub star_rating: usize,
}

impl GameState {
    pub fn new() -> Self {
        let levels = get_levels();
        let level_idx = load_last_level_reached().min(levels.len().saturating_sub(1));
        let level = levels[level_idx].clone();
        let grid = level.create_initial_grid();
        let walls_left = level.walls_per_turn;

        Self {
            levels,
            current_level_idx: level_idx,
            level,
            grid,
            turn_number: 1,
            walls_left,
            phase: GamePhase::PlayerTurn,
            placed_walls_this_turn: Vec::new(),
            anim_timer: 0.0,
            expansion_steps: Vec::new(),
            current_anim_step: 0,
            dying_biomass: Vec::new(),
            newly_cloned_this_step: Vec::new(),
            newly_starved_this_step: Vec::new(),
            star_rating: 3,
        }
    }

    pub fn load_level(&mut self, level_idx: usize) {
        if level_idx < self.levels.len() {
            save_last_level_reached(level_idx);
            self.init_level(level_idx);
        }
    }

    pub fn reset_level(&mut self) {
        self.init_level(self.current_level_idx);
    }

    fn init_level(&mut self, level_idx: usize) {
        self.current_level_idx = level_idx;
        self.level = self.levels[level_idx].clone();
        self.grid = self.level.create_initial_grid();
        self.turn_number = 1;
        self.walls_left = self.level.walls_per_turn;
        self.phase = GamePhase::PlayerTurn;
        self.placed_walls_this_turn.clear();
        self.expansion_steps.clear();
        self.current_anim_step = 0;
        self.dying_biomass.clear();
        self.newly_cloned_this_step.clear();
        self.newly_starved_this_step.clear();
        self.anim_timer = 0.0;
        self.star_rating = 3;
    }

    pub fn try_place_wall(&mut self, edge: Edge) -> bool {
        if self.phase != GamePhase::PlayerTurn || self.walls_left == 0 {
            return false;
        }

        if self.grid.can_place_wall(edge) {
            self.grid.set_edge(edge, EdgeState::Wall);
            self.placed_walls_this_turn.push(edge);
            self.walls_left -= 1;

            if self.walls_left == 0 {
                self.end_turn();
            }
            true
        } else {
            false
        }
    }

    pub fn remove_placed_wall(&mut self, edge: Edge) -> bool {
        if self.phase != GamePhase::PlayerTurn {
            return false;
        }

        if let Some(pos) = self.placed_walls_this_turn.iter().position(|&e| e == edge) {
            self.placed_walls_this_turn.remove(pos);
            self.grid.set_edge(edge, EdgeState::Passable);
            self.walls_left += 1;
            true
        } else {
            false
        }
    }

    pub fn end_turn(&mut self) {
        if self.phase != GamePhase::PlayerTurn {
            return;
        }

        // Calculate biomass expansion steps (capped at walls_per_turn * 2 per turn)
        let max_clones = self.level.walls_per_turn * 2;
        self.expansion_steps =
            expand_biomass_step_by_step(&self.grid, self.level.spread_steps, max_clones);
        self.current_anim_step = 0;
        self.anim_timer = 0.0;

        if self.expansion_steps.is_empty() {
            // Immediately apply expansion if no steps
            self.apply_all_expansion();
            self.start_isolation_phase();
        } else {
            self.phase = GamePhase::BiomassExpansion;
        }
    }

    fn apply_all_expansion(&mut self) {
        for step in &self.expansion_steps {
            for event in step {
                self.grid
                    .set_cell(event.to.0, event.to.1, CellType::Biomass);
                self.newly_cloned_this_step.push(*event);
            }
        }
    }

    pub fn update(&mut self, dt: f32) -> Option<SoundTrigger> {
        let mut sound_trigger = None;

        match self.phase {
            GamePhase::BiomassExpansion => {
                let step_duration = 0.35;
                self.anim_timer += dt;

                if self.anim_timer >= step_duration {
                    self.anim_timer = 0.0;
                    if self.current_anim_step < self.expansion_steps.len() {
                        for event in &self.expansion_steps[self.current_anim_step] {
                            self.grid
                                .set_cell(event.to.0, event.to.1, CellType::Biomass);
                            self.newly_cloned_this_step.push(*event);
                        }
                        sound_trigger = Some(SoundTrigger::BiomassTick);
                        self.current_anim_step += 1;
                    }

                    if self.current_anim_step >= self.expansion_steps.len() {
                        self.start_isolation_phase();
                    }
                }
            }
            GamePhase::IsolationCheck => {
                // Apply starvation deactivation
                for &(r, c) in &self.dying_biomass {
                    self.grid.set_cell(r, c, CellType::Empty);
                }

                if !self.dying_biomass.is_empty() {
                    sound_trigger = Some(SoundTrigger::IsolationPop);
                }
                self.dying_biomass.clear();

                // Evaluate terminal conditions
                let biomass_count = self.grid.count_biomass();

                if biomass_count == 0 {
                    self.phase = GamePhase::Victory;
                    let (three_star, two_star) = self.level.star_thresholds;
                    self.star_rating = if self.turn_number <= three_star {
                        3
                    } else if self.turn_number <= two_star {
                        2
                    } else {
                        1
                    };
                    let is_last_level = self.current_level_idx + 1 >= self.levels.len();
                    sound_trigger = if is_last_level {
                        Some(SoundTrigger::GrandFanfare)
                    } else {
                        Some(SoundTrigger::WinFanfare)
                    };
                } else if biomass_count >= self.level.max_threshold
                    || (!self.grid.has_any_legal_wall_placement() && biomass_count > 0)
                {
                    self.phase = GamePhase::Defeat;
                    sound_trigger = Some(SoundTrigger::LossAlert);
                } else {
                    // Advance to next player turn
                    self.turn_number += 1;
                    self.walls_left = self.level.walls_per_turn;
                    self.placed_walls_this_turn.clear();
                    self.phase = GamePhase::PlayerTurn;
                }
            }
            GamePhase::PlayerTurn | GamePhase::Victory | GamePhase::Defeat => {}
        }

        sound_trigger
    }

    fn start_isolation_phase(&mut self) {
        self.phase = GamePhase::IsolationCheck;
        self.dying_biomass = evaluate_sealed_enclosure_dieoff(&self.grid);
        self.newly_starved_this_step = self.dying_biomass.clone();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundTrigger {
    WallPlace,
    BiomassTick,
    IsolationPop,
    WinFanfare,
    GrandFanfare,
    LossAlert,
    ButtonClick,
    InvalidMove,
}
