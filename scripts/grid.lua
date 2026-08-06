-- Grid module for Biomass game logic
-- Implements grid topology, edge barricades, BFS spread, component isolation flood-fill, and win/loss rules.

local M = {}

-- Create a new Grid instance
function M.create(rows, cols, n_walls_per_turn, n_steps_spread, max_threshold)
    local self = {
        rows = rows or 6,
        cols = cols or 6,
        n_walls_per_turn = n_walls_per_turn or 2,
        n_steps_spread = n_steps_spread or 1,
        max_threshold = max_threshold or (rows * cols * 0.8),
        turn = 1,
        walls_placed_this_turn = 0,
        turn_history = {}, -- list of wall placements in current turn for undo
        cells = {},
        h_edges = {}, -- h_edges[r][c] between (r,c) and (r+1,c) (0 .. rows-2, 0 .. cols-1)
        v_edges = {}  -- v_edges[r][c] between (r,c) and (r,c+1) (0 .. rows-1, 0 .. cols-2)
    }

    -- Initialize cells
    for r = 0, self.rows - 1 do
        self.cells[r] = {}
        for c = 0, self.cols - 1 do
            self.cells[r][c] = { state = 0 } -- 0: Empty, 1: Biomass
        end
    end

    -- Initialize horizontal edges
    for r = 0, self.rows - 2 do
        self.h_edges[r] = {}
        for c = 0, self.cols - 1 do
            self.h_edges[r][c] = 0 -- 0: Passable, 1: Wall
        end
    end

    -- Initialize vertical edges
    for r = 0, self.rows - 1 do
        self.v_edges[r] = {}
        for c = 0, self.cols - 2 do
            self.v_edges[r][c] = 0 -- 0: Passable, 1: Wall
        end
    end

    return setmetatable(self, { __index = M })
end

-- Load initial seed biomass and pre-placed walls from level config
function M:load_level(config)
    self.rows = config.rows or self.rows
    self.cols = config.cols or self.cols
    self.n_walls_per_turn = config.n_walls_per_turn or 2
    self.n_steps_spread = config.n_steps_spread or 1
    self.max_threshold = config.max_threshold or math.floor(self.rows * self.cols * 0.75)
    self.turn = 1
    self.walls_placed_this_turn = 0
    self.turn_history = {}

    -- Re-init tables if size changed
    for r = 0, self.rows - 1 do
        self.cells[r] = {}
        for c = 0, self.cols - 1 do
            self.cells[r][c] = { state = 0 }
        end
    end

    for r = 0, self.rows - 2 do
        self.h_edges[r] = {}
        for c = 0, self.cols - 1 do
            self.h_edges[r][c] = 0
        end
    end

    for r = 0, self.rows - 1 do
        self.v_edges[r] = {}
        for c = 0, self.cols - 2 do
            self.v_edges[r][c] = 0
        end
    end

    -- Seed biomass
    if config.biomass_seeds then
        for _, pos in ipairs(config.biomass_seeds) do
            local r, c = pos[1], pos[2]
            if r >= 0 and r < self.rows and c >= 0 and c < self.cols then
                self.cells[r][c].state = 1
            end
        end
    end

    -- Initial walls
    if config.initial_h_walls then
        for _, e in ipairs(config.initial_h_walls) do
            local r, c = e[1], e[2]
            if self.h_edges[r] and self.h_edges[r][c] then
                self.h_edges[r][c] = 1
            end
        end
    end

    if config.initial_v_walls then
        for _, e in ipairs(config.initial_v_walls) do
            local r, c = e[1], e[2]
            if self.v_edges[r] and self.v_edges[r][c] then
                self.v_edges[r][c] = 1
            end
        end
    end
end

-- Helper: Check if edge between (r1,c1) and (r2,c2) is open (0: passable, 1: wall)
function M:is_edge_open(r1, c1, r2, c2)
    -- Check bounds
    if r1 < 0 or r1 >= self.rows or c1 < 0 or c1 >= self.cols then return false end
    if r2 < 0 or r2 >= self.rows or c2 < 0 or c2 >= self.cols then return false end

    if r1 == r2 then
        -- Vertical edge (between columns)
        local min_c = math.min(c1, c2)
        if min_c >= 0 and min_c <= self.cols - 2 then
            return self.v_edges[r1][min_c] == 0
        end
    elseif c1 == c2 then
        -- Horizontal edge (between rows)
        local min_r = math.min(r1, r2)
        if min_r >= 0 and min_r <= self.rows - 2 then
            return self.h_edges[min_r][c1] == 0
        end
    end

    return false
end

-- Toggle or place wall on edge
function M:place_wall(edge_type, r, c)
    if self.walls_placed_this_turn >= self.n_walls_per_turn then
        return false, "No walls remaining this turn"
    end

    if edge_type == "h" then
        if r >= 0 and r < self.rows - 1 and c >= 0 and c < self.cols then
            if self.h_edges[r][c] == 0 then
                self.h_edges[r][c] = 1
                self.walls_placed_this_turn = self.walls_placed_this_turn + 1
                table.insert(self.turn_history, { type = "h", r = r, c = c })
                return true
            end
        end
    elseif edge_type == "v" then
        if r >= 0 and r < self.rows and c >= 0 and c < self.cols - 1 then
            if self.v_edges[r][c] == 0 then
                self.v_edges[r][c] = 1
                self.walls_placed_this_turn = self.walls_placed_this_turn + 1
                table.insert(self.turn_history, { type = "v", r = r, c = c })
                return true
            end
        end
    end

    return false, "Edge already has a wall or out of bounds"
end

-- Undo last wall placed in current turn
function M:undo_wall()
    if #self.turn_history == 0 then
        return false, "Nothing to undo"
    end

    local last = table.remove(self.turn_history)
    if last.type == "h" then
        self.h_edges[last.r][last.c] = 0
    elseif last.type == "v" then
        self.v_edges[last.r][last.c] = 0
    end

    self.walls_placed_this_turn = math.max(0, self.walls_placed_this_turn - 1)
    return true
end

-- Get neighbors reachable across open edges
function M:get_passable_neighbors(r, c)
    local neighbors = {}
    local dirs = { {-1, 0}, {1, 0}, {0, -1}, {0, 1} }
    for _, d in ipairs(dirs) do
        local nr, nc = r + d[1], c + d[2]
        if self:is_edge_open(r, c, nr, nc) then
            table.insert(neighbors, { r = nr, c = nc })
        end
    end
    return neighbors
end

-- Biomass Phase: Spread up to N_steps
-- Returns table of steps, where each step contains newly infected cells {r, c}
function M:spread_biomass()
    local spread_steps = {}

    for _ = 1, self.n_steps_spread do
        local newly_infected = {}
        local visited_this_step = {}

        -- Find active biomass cells
        for r = 0, self.rows - 1 do
            for c = 0, self.cols - 1 do
                if self.cells[r][c].state == 1 then
                    local neighbors = self:get_passable_neighbors(r, c)
                    for _, n in ipairs(neighbors) do
                        if self.cells[n.r][n.c].state == 0 then
                            local key = n.r .. "_" .. n.c
                            if not visited_this_step[key] then
                                visited_this_step[key] = true
                                table.insert(newly_infected, { r = n.r, c = n.c })
                            end
                        end
                    end
                end
            end
        end

        if #newly_infected == 0 then
            break -- No further spread possible
        end

        -- Apply infection for this step
        for _, cell in ipairs(newly_infected) do
            self.cells[cell.r][cell.c].state = 1
        end

        table.insert(spread_steps, newly_infected)
    end

    return spread_steps
end

-- Isolation Phase: Die-off under Sealed Enclosure Rule
-- Evaluates connected components of biomass.
-- If a component cannot reach ANY empty cell (state == 0) across open edges, it dies!
-- Returns table of died cells {r, c}
function M:evaluate_isolation()
    local died_cells = {}
    local visited = {}

    for r = 0, self.rows - 1 do
        for c = 0, self.cols - 1 do
            local key = r .. "_" .. c
            if self.cells[r][c].state == 1 and not visited[key] then
                -- BFS to gather the biomass connected component
                local component = {}
                local queue = { { r = r, c = c } }
                visited[key] = true

                local reachable_empty_cells = 0

                -- BFS queue loop
                local head = 1
                while head <= #queue do
                    local curr = queue[head]
                    head = head + 1
                    table.insert(component, curr)

                    local neighbors = self:get_passable_neighbors(curr.r, curr.c)
                    for _, n in ipairs(neighbors) do
                        if self.cells[n.r][n.c].state == 0 then
                            -- Found reachable empty cell
                            reachable_empty_cells = reachable_empty_cells + 1
                        elseif self.cells[n.r][n.c].state == 1 then
                            local nkey = n.r .. "_" .. n.c
                            if not visited[nkey] then
                                visited[nkey] = true
                                table.insert(queue, { r = n.r, c = n.c })
                            end
                        end
                    end
                end

                -- If component has NO reachable empty cells, it is isolated and dies!
                if reachable_empty_cells == 0 then
                    for _, cell in ipairs(component) do
                        self.cells[cell.r][cell.c].state = 0 -- deactivates (dies)
                        table.insert(died_cells, { r = cell.r, c = cell.c })
                    end
                end
            end
        end
    end

    return died_cells
end

-- Advance turn cycle
function M:end_turn()
    -- Phase 2: Spread
    local spread_steps = self:spread_biomass()

    -- Phase 3: Isolation
    local died_cells = self:evaluate_isolation()

    -- Reset turn wall placement counter and history
    self.turn = self.turn + 1
    self.walls_placed_this_turn = 0
    self.turn_history = {}

    -- Check win/loss
    local status = self:check_status()

    return {
        spread_steps = spread_steps,
        died_cells = died_cells,
        status = status
    }
end

-- Count active biomass cells
function M:get_biomass_count()
    local count = 0
    for r = 0, self.rows - 1 do
        for c = 0, self.cols - 1 do
            if self.cells[r][c].state == 1 then
                count = count + 1
            end
        end
    end
    return count
end

-- Count open (passable) edges remaining
function M:get_open_edges_count()
    local count = 0
    for r = 0, self.rows - 2 do
        for c = 0, self.cols - 1 do
            if self.h_edges[r][c] == 0 then count = count + 1 end
        end
    end
    for r = 0, self.rows - 1 do
        for c = 0, self.cols - 2 do
            if self.v_edges[r][c] == 0 then count = count + 1 end
        end
    end
    return count
end

-- Check game terminal status
-- Returns: "ongoing", "win", or "loss"
function M:check_status()
    local biomass_count = self:get_biomass_count()

    if biomass_count == 0 then
        return "win"
    end

    if biomass_count >= self.max_threshold then
        return "loss"
    end

    -- Check if no legal wall placement remains while biomass is active
    local open_edges = self:get_open_edges_count()
    if open_edges == 0 and biomass_count > 0 then
        return "loss"
    end

    return "ongoing"
end

return M
