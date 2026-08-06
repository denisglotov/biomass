# Biomass - Sci-Fi Turn-Based Containment Strategy Game

**Biomass** is a turn-based strategy puzzle game developed for the **Defold Game Engine**. The player commands containment forces on a 2D grid facility to trap, isolate, and neutralize expanding sci-fi bio-hazards.


## 🎮 Game Rules & Mechanics

### Environment & Components
- **Facility Grid**: $M \times N$ matrix of cells $(r, c)$.
- **Cell Component State**: `0` (Empty) or `1` (Active Biomass).
- **Edge Barricades**: 4 borders per cell. Edge value `0` (Passable) or `1` (Wall).

### Turn Loop (Turn-Based Strategy)

1. **Player Phase (Barricade Deployment)**
   - Place up to $N_{\text{walls}}$ barricade walls on open passable edges per turn.
   - Includes **Undo Wall Placement** ($Z$) and **Reset Level** ($R$).
   - Click the **End Turn** button or press Space to trigger the biomass spread.

2. **Biomass Phase (Spread Expansion)**
   - Active biomass cells expand up to $N_{\text{steps}}$ distance using Breadth-First Search (BFS).
   - Infection transfers across adjacent cells if and only if the shared edge is passable (`0` / no wall).

3. **Isolation Phase (Die-off via Sealed Enclosure Rule)**
   - Evaluates connected components of biomass across passable edges.
   - **Sealed Enclosure Rule**: A biomass component dies off if it has no open path across passable edges to ANY empty cell (`0`) anywhere on the grid.
   - When trapped inside a wall enclosure with no free empty cells left to infect, the biomass starves and deactivates.

### Terminal Conditions
- **Win (+1 Reward)**: All active biomass cells are deactivated (0 remaining on grid).
- **Loss (-1 Reward)**: Biomass count reaches/exceeds `MaxThreshold`.


## 🚀 How to Build & Play

You can play and build the game using the powerful Defold Editor, or from the terminal using the provided `Makefile` toolchain.

### Option 1: Native Defold Editor (Recommended)
1. Open the **Defold Editor**.
2. Select **Open Project** and navigate to this repository (`biomass`).
3. Select `game.project`.
4. Press **Project ➔ Build** (`Cmd+B` on macOS / `Ctrl+B` on Windows) to run the game natively.
5. Select **Project ➔ Bundle ➔ HTML5** to export for the web.

### Option 2: Terminal (Makefile)
The project is entirely standalone and does not require Node.js or NPM. To build the WebAssembly bundle headless via the terminal on macOS:

1. **Setup**: Downloads the headless Defold compiler (`bob.jar`).
```bash
make setup
```

2. **Build**: Compiles the Lua scripts and creates the WebAssembly bundle in `dist/`.
```bash
make build
```

3. **Serve**: Spins up a local Python web server on port 3000 to test the game in your browser.
```bash
make start
```


## 🎵 Audio Engine
The game utilizes procedurally generated 8-bit retro sound effects:
- `click.wav`: UI Interaction
- `wall.wav`: Barricade Deployment
- `spread.wav`: Biomass Expansion
- `win.wav`: Sector Cleared
- `loss.wav`: Containment Breach

These sounds were synthesized via Python and integrated directly into Defold's Sound Component architecture (`scripts/sound.lua`).


## 🔍 IDE Language Server Configuration
The repository includes a local [`.luarc.json`](.luarc.json) file.
- **Purpose**: Configures IDE Lua Language Servers (LuaLS) to recognize Defold engine C++ runtime globals (`sound`, `msg`, `hash`, `go`, `gui`, `render`, `vmath`, etc.) and lifecycle callbacks (`init`, `on_input`, `on_message`).
- **Offline Compatibility**: Formatted locally without external `$schema` network download requirements.
