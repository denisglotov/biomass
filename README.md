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

---

## 📱 Building for Android

### Prerequisites

**Android SDK & NDK**

Install via [Android command-line tools](https://developer.android.com/studio#command-line-tools-only) or Homebrew:

```bash
brew install --cask android-commandlinetools
sdkmanager "platforms;android-35" "ndk;26.1.10909125" "build-tools;35.0.0"
```

> The minimum tested NDK version is **r26**. NDK r26+ ships LLVM-only toolchains
> (no legacy GNU binutils). `cargo-quad-apk` from git handles this correctly.

**cargo-quad-apk (from git — required)**

The published crates.io version (`0.1.4`, 2022) does not support NDK r26+, JDK 17+,
or Android API 31+ manifest requirements. Install from the upstream git repo instead:

```bash
git clone https://github.com/not-fl3/cargo-quad-apk
cargo install --path ./cargo-quad-apk --force
```

**Rust Android target**

```bash
rustup target add aarch64-linux-android
```

**JDK 17+** (JDK 8 is no longer required)

```bash
brew install --cask temurin  # or any JDK 17+
```

### Building

```bash
ANDROID_HOME=/opt/homebrew/share/android-commandlinetools \
  cargo quad-apk build --release
```

The APK will be created at:
```
target/android-artifacts/release/apk/biomass.apk
```

### Installing & Running

```bash
# List connected devices
adb devices

# Install (replace the device serial as needed)
adb -s <device-serial> install -r target/android-artifacts/release/apk/biomass.apk

# Launch
adb -s <device-serial> shell am start -n org.dymka.biomass/.MainActivity

# View logs
adb -s <device-serial> logcat -s biomass
```

### Cargo.toml Android metadata

The Android package name, version, and target API are configured in [`Cargo.toml`](Cargo.toml)
under the `[package.metadata.android]` section.
