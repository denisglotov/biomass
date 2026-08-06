-- Level Manager for Biomass
-- Manages level data, campaign progression, level loading, star rating calculations.

local M = {}

M.levels = {
    {
        id = 1,
        title = "Containment 101",
        description = "Learn the basics of placing barricades to trap and isolate biomass.",
        rows = 4,
        cols = 4,
        n_walls_per_turn = 2,
        n_steps_spread = 1,
        max_threshold = 12,
        biomass_seeds = { {1, 1} },
        initial_h_walls = {},
        initial_v_walls = {},
        target_turns_3star = 3,
        target_turns_2star = 5
    },
    {
        id = 2,
        title = "Twin Spores",
        description = "Two biomass clusters are expanding simultaneously. Contain both!",
        rows = 5,
        cols = 5,
        n_walls_per_turn = 2,
        n_steps_spread = 1,
        max_threshold = 18,
        biomass_seeds = { {1, 1}, {3, 3} },
        initial_h_walls = {},
        initial_v_walls = {},
        target_turns_3star = 4,
        target_turns_2star = 6
    },
    {
        id = 3,
        title = "Divided Sectors",
        description = "Use pre-placed barricades to channel and seal off the bio-hazard.",
        rows = 6,
        cols = 6,
        n_walls_per_turn = 2,
        n_steps_spread = 1,
        max_threshold = 26,
        biomass_seeds = { {0, 2}, {5, 3} },
        initial_h_walls = { {2, 1}, {2, 2}, {2, 3} },
        initial_v_walls = { {1, 2}, {4, 2} },
        target_turns_3star = 5,
        target_turns_2star = 8
    },
    {
        id = 4,
        title = "Rapid Mutation",
        description = "WARNING: Biomass expands 2 steps per turn! Act quickly.",
        rows = 6,
        cols = 6,
        n_walls_per_turn = 2,
        n_steps_spread = 2,
        max_threshold = 24,
        biomass_seeds = { {2, 2}, {3, 3} },
        initial_h_walls = {},
        initial_v_walls = {},
        target_turns_3star = 4,
        target_turns_2star = 7
    },
    {
        id = 5,
        title = "Corridor Siege",
        description = "A 7x7 facility under siege. Multi-flank containment required.",
        rows = 7,
        cols = 7,
        n_walls_per_turn = 3,
        n_steps_spread = 1,
        max_threshold = 35,
        biomass_seeds = { {1, 1}, {1, 5}, {5, 3} },
        initial_h_walls = { {3, 1}, {3, 5} },
        initial_v_walls = { {1, 3}, {5, 3} },
        target_turns_3star = 6,
        target_turns_2star = 9
    },
    {
        id = 6,
        title = "Infection Wave",
        description = "Fast-spreading biomass clusters across an 8x8 grid.",
        rows = 8,
        cols = 8,
        n_walls_per_turn = 3,
        n_steps_spread = 2,
        max_threshold = 45,
        biomass_seeds = { {2, 2}, {2, 5}, {5, 2}, {5, 5} },
        initial_h_walls = {},
        initial_v_walls = {},
        target_turns_3star = 7,
        target_turns_2star = 11
    },
    {
        id = 7,
        title = "Bio-Reactor Breach",
        description = "A central reactor breach surrounded by multiple spore pockets.",
        rows = 8,
        cols = 8,
        n_walls_per_turn = 3,
        n_steps_spread = 2,
        max_threshold = 48,
        biomass_seeds = { {3, 3}, {3, 4}, {4, 3}, {4, 4}, {0, 0} },
        initial_h_walls = { {1, 3}, {5, 3} },
        initial_v_walls = { {3, 1}, {3, 5} },
        target_turns_3star = 8,
        target_turns_2star = 12
    },
    {
        id = 8,
        title = "Outbreak Zero",
        description = "The ultimate containment challenge on a 10x10 facility grid.",
        rows = 10,
        cols = 10,
        n_walls_per_turn = 4,
        n_steps_spread = 2,
        max_threshold = 70,
        biomass_seeds = { {1, 1}, {1, 8}, {8, 1}, {8, 8}, {4, 4}, {5, 5} },
        initial_h_walls = {},
        initial_v_walls = {},
        target_turns_3star = 9,
        target_turns_2star = 14
    }
}

function M.get_level(id)
    for _, lvl in ipairs(M.levels) do
        if lvl.id == id then
            return lvl
        end
    end
    return M.levels[1]
end

function M.get_total_levels()
    return #M.levels
end

function M.calculate_stars(level_config, turns_taken)
    if turns_taken <= level_config.target_turns_3star then
        return 3
    elseif turns_taken <= level_config.target_turns_2star then
        return 2
    else
        return 1
    end
end

return M
