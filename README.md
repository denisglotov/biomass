# Biomass - Sci-Fi Turn-Based Containment Strategy Game

**Biomass** is an HTML5 Canvas turn-based strategy puzzle game. The player commands containment forces on a 2D grid facility to trap, isolate, and neutralize expanding sci-fi bio-hazards.


## 🎮 Game Rules & Mechanics

### Environment & Components
- **Facility Grid**: $M \times N$ matrix of cells $(r, c)$.
- **Cell Component State**: `0` (Empty) or `1` (Active Biomass).
- **Edge Barricades**: 4 borders per cell. Edge value `0` (Passable) or `1` (Wall).

### Turn Loop (Turn-Based Strategy)

1. **Player Phase (Barricade Deployment)**
   - Place up to $N_{\text{walls}}$ barricade walls on open passable edges per turn.
   - Includes **Undo Wall Placement** (`Z`) and **Reset Level** (`R`).
   - Press **End Turn** (Space) or auto-advance when wall placement limit is reached.

2. **Biomass Phase (Spread Expansion)**
   - Active biomass cells expand up to $N_{\text{steps}}$ distance using Breadth-First Search (BFS).
   - Infection transfers across adjacent cells if and only if the shared edge is passable (`0` / no wall).
   - Animated step-by-step with speed controls (**1x**, **2x**, **Skip**).

3. **Isolation Phase (Die-off via Sealed Enclosure Rule)**
   - Evaluates connected components of biomass across passable edges.
   - **Sealed Enclosure Rule**: A biomass component dies off if it has no open path across passable edges to ANY empty cell (`0`) anywhere on the grid.
   - When trapped inside a wall enclosure with no free empty cells left to infect, the biomass starves and deactivates.

### Terminal Conditions
- **Win (+1 Reward)**: All active biomass cells are deactivated (0 remaining on grid).
- **Loss (-1 Reward)**: Biomass count reaches/exceeds `MaxThreshold` OR no legal wall placement remains while active biomass exists.


## 🚀 How to Run & Play

Run the interactive web server:
```bash
npm start
```
Or open `index.html` directly in any web browser.


## 🔍 Code Quality & Linter

Run automated linting:
```bash
npm run lint
```
- **JavaScript**: Linted using ESLint ([`.eslintrc.json`](.eslintrc.json)).

### Continuous Integration (GitHub Actions)
Continuous integration is configured via [`.github/workflows/lint.yml`](.github/workflows/lint.yml). On every `push` and `pull_request`, GitHub Actions automatically installs Node.js dependencies and executes `npm run lint`.
