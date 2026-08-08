# Biomass - Sci-Fi Turn-Based Containment Strategy Game (Rust + WebAssembly)

**Biomass** is a tactical turn-based strategy puzzle game built entirely in **Rust** using **Macroquad** and compiled to **WebAssembly** (`wasm32-unknown-unknown`).

The player commands facility containment forces on a 2D grid to trap, isolate, and neutralize expanding sci-fi bio-hazards before grid capacity limits are breached.

---

## 🎮 Game Rules & Mechanics

### Environment & Components
- **Facility Grid**: $M \times N$ matrix of cells $(r, c)$.
- **Cell Component State**: `Empty`, `Active Biomass`, or **`Impassable Obstacle`** (structural pillars/rocks blocking both biomass spread and wall placement).
- **Edge Barricades**: 4 borders per cell. Edge value `Passable` or `Barricade Wall`.

### Turn Loop (Turn-Based Strategy)

1. **Player Phase (Barricade Deployment)**
   - Place up to $N_{\text{walls}}$ barricade walls on open passable edges per turn.
   - Includes **Undo Wall Placement** (`Z`) and **Reset Level** (`R`).
   - Press **End Turn** (Space) or auto-advance when wall placement limit is reached.

2. **Biomass Phase (Spread Expansion)**
   - Active biomass cells expand up to $N_{\text{steps}}$ distance using Breadth-First Search (BFS).
   - Infection transfers across adjacent cells if and only if the shared edge is passable (no wall) and target cell is not an obstacle.
   - Animated step-by-step with speed controls (**1x**, **2x**, **Skip**).

3. **Isolation Phase (Die-off via Sealed Enclosure Rule)**
   - Evaluates connected components of biomass across passable edges.
   - **Sealed Enclosure Rule**: A biomass component dies off if it has no open path across passable edges to ANY empty cell anywhere on the grid.
   - When trapped inside a wall enclosure with no free empty cells left to infect, the biomass starves and deactivates.

### Terminal Conditions
- **Victory**: All active biomass cells are deactivated (0 remaining on grid). Calculates 1-3 star performance rating based on turns taken.
- **Defeat**: Biomass count reaches/exceeds `MaxThreshold` OR no legal wall placement remains while active biomass exists.

---

## 🛠️ Quickstart with Justfile

This repository uses [`just`](Justfile) for all build, run, and testing workflows.

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (1.80+)
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- [Just task runner](https://github.com/casey/just): `brew install just`

### Common Workflows

```bash
# List all available recipes
just

# Build release WebAssembly target (creates biomass.wasm in root)
just build-wasm

# Serve the WebAssembly game locally on http://localhost:8080
just serve

# Run code formatting check
just fmt-check

# Run strict Clippy lints
just clippy

# Run full CI validation suite (formatting, clippy, WASM build)
just ci
```

---

## 🔍 Code Quality & Continuous Integration

Run the complete automated quality suite locally:
```bash
just ci
```

### Continuous Integration (GitHub Actions)
Continuous integration is configured via [`.github/workflows/lint.yml`](.github/workflows/lint.yml). On every `push` and `pull_request`, GitHub Actions installs `just` and executes `just ci` to verify formatting, Clippy lints, and WebAssembly compilation.
