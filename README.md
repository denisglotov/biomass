# Biomass - Sci-Fi Turn-Based Containment Strategy Game

**Biomass** is a tactical turn-based strategy puzzle game built in **Rust** using **Macroquad**, supporting
**WebAssembly** (`wasm32-unknown-unknown`), **Desktop**, and **Android**.

The player commands facility containment forces on a 2D grid to trap, isolate, and neutralize expanding sci-fi
bio-hazards before grid capacity limits are breached.

---

## 🎮 Game Rules & Mechanics

### Environment & Components
- **Facility Grid**: $M \times N$ matrix of cells $(r, c)$.
- **Cell Component State**: `Empty`, `Active Biomass`, or **`Impassable Obstacle`** (structural pillars/rocks blocking both biomass spread and wall placement).
- **Edge Barricades**: 4 borders per cell. Edge value `Passable` or `Barricade Wall`.

### Turn Loop (Turn-Based Strategy)

1. **Player Phase (Barricade Deployment)**
   - Place up to $N_{\text{walls}}$ barricade walls on open passable edges per turn.
   - Walls placed during the current turn appear in **Hazard Amber ("In-Construction")** state with an animated construction aura.
   - Click an **in-construction wall** again to remove it and refund the wall point.
   - Turns automatically advance to the Biomass phase once all allocated walls for the turn are deployed.

2. **Biomass Phase (Spread Expansion)**
   - Each active biomass cell produces **one new cell per step** (randomly into an adjacent free cell unblocked by walls or obstacles).
   - Infection expands up to $N_{\text{steps}}$ distance per turn, capped at a maximum clone budget ($2 \times N_{\text{walls}}$).
   - Step-by-step visual simulation featuring viscous fluid droplet jumps, mitosis shockwaves, and surface splatter beads.

3. **Isolation Phase (Die-off via Sealed Enclosure Rule)**
   - Evaluates connected components of biomass across passable edges.
   - **Sealed Enclosure Rule**: A biomass component dies off if it has no open path across passable edges to ANY empty cell anywhere on the grid.
   - When trapped inside a wall enclosure with no free empty cells left to infect, the biomass starves and deactivates.

### Terminal Conditions
- **Victory**: All active biomass cells are deactivated (0 remaining on grid). Calculates a 1-3 star performance rating based on turns taken.
- **Defeat**: Biomass count reaches/exceeds `MaxThreshold` OR no legal wall placement remains while active biomass exists.

---

## 🛠️ Quickstart with Justfile

This repository uses [`Justfile`](Justfile) for all build, run, and testing workflows.

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (1.80+)
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- [Just task runner](https://github.com/casey/just): `brew install just`

### Common Workflows

```bash
# List all available recipes
just

# Run the native desktop application (uses system language by default)
just run

# Run with an explicit language override
just run --lang ru-RU   # Russian
just run -l es-ES       # Spanish
just run --lang=de-DE   # German
just run -l fr-FR       # French


# Build release WebAssembly target
just build-wasm

# Install WASM binary to web directory
just install-wasm

# Serve the WebAssembly game locally on http://localhost:8080
just serve

# Run unit tests
just test

# Check formatting and Clippy lints
just fmt-check
just clippy

# Run complete CI test suite (formatting, clippy, tests, WASM build)
just ci
```

---

## 🔍 Code Quality & Continuous Integration

Run the complete automated quality suite locally:
```bash
just ci
```

### Continuous Integration & Releases (GitHub Actions)

- **CI Suite ([`.github/workflows/lint.yml`](.github/workflows/lint.yml))**: On every `push` and `pull_request` to
  `master`, runs formatting (`rustfmt`), Clippy lints, unit tests, and WebAssembly build.
- **Android Release Pipeline ([`.github/workflows/release.yml`](.github/workflows/release.yml))**: On every version tag
  push (`v*`) or manual `workflow_dispatch`, builds, packages, and signs both the release Android APK and App Bundle
  (`.aab`), uploading them to a GitHub Release with auto-generated release notes.
  - Required Secrets / Variables: `KEYSTORE_BASE64`, `KEYSTORE_PASSWORD`, `KEYSTORE_ALIAS`.

## 📱 Building for Android

### Prerequisites

**Android SDK & NDK**

Install via [Android command-line tools](https://developer.android.com/studio#command-line-tools-only) or Homebrew:

```bash
brew install --cask android-commandlinetools
sdkmanager "platforms;android-36" "ndk;26.1.10909125" "build-tools;36.0.0"
```

> The minimum tested NDK version is **r26**. NDK r26+ ships LLVM-only toolchains (no legacy GNU binutils). `cargo-quad-apk` from git handles this correctly.

**cargo-quad-apk (from git — required)**

The published crates.io version (`0.1.4`, 2022) does not support NDK r26+, JDK 17+, or Android API 31+ manifest requirements. Install from the upstream git repo instead:

```bash
git clone https://github.com/not-fl3/cargo-quad-apk
cargo install --path ./cargo-quad-apk --force
```

**Rust Android target**

```bash
rustup target add aarch64-linux-android
```

**JDK**

```bash
brew install --cask temurin@21  # or any JDK 17+
```

### Building APK & Android App Bundle (.aab)

```bash
# Build release APK
ANDROID_HOME=/opt/homebrew/share/android-commandlinetools just build-android

# Build release Android App Bundle (.aab) for Google Play Store
ANDROID_HOME=/opt/homebrew/share/android-commandlinetools just build-aab
```

Artifact outputs:
- **APK**: `target/android-artifacts/release/apk/biomass.apk`
- **AAB**: `target/android-artifacts/release/apk/biomass.aab`

---

### 🔑 Signing for Production Release

#### 1. Generate Release Keystore (One-Time)
```bash
keytool -genkeypair -v -keystore release.keystore -alias biomass -keyalg RSA -keysize 2048 -validity 10000
```

#### 2. Sign APK (`apksigner`)
```bash
apksigner sign --ks release.keystore --ks-key-alias biomass \
  --out target/android-artifacts/release/apk/biomass-signed.apk \
  target/android-artifacts/release/bin/biomass/biomass_unaligned.apk
```

#### 3. Sign Android App Bundle (`jarsigner`)
```bash
jarsigner -keystore release.keystore target/android-artifacts/release/apk/biomass.aab biomass
jarsigner -verify target/android-artifacts/release/apk/biomass.aab
```

### Installing & Running APK

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

### Cargo.toml Android Metadata

The Android package name, version, and target API are configured in [`Cargo.toml`](Cargo.toml) under the `[package.metadata.android]` section.

---

## 🌐 Internationalization & Locales

Biomass supports full localization with standard BCP-47 region tag resolution:
- **English (US)**: `en-US` (Default)
- **Russian**: `ru-RU`
- **Spanish**: `es-ES`
- **German**: `de-DE`
- **French**: `fr-FR`
- **Japanese**: `ja-JP`
- **Chinese (Simplified)**: `zh-CN`
- **Korean**: `ko-KR`

### Locale Resolution & Platform Integration
- **WebAssembly (WASM)**: Automatically detects browser/device locale via `navigator.language`.
- **Android**: Automatically detects system and Android 13+ per-app locale via JNI `Locale.getDefault().toLanguageTag()` backed by `res/xml/locales_config.xml`.
- **Desktop (Native)**: Automatically detects OS locale (macOS CoreFoundation, Windows API, POSIX environment variables) or accepts CLI flag overrides (e.g. `just run --lang ja-JP`, `just run -l zh-CN`, or `just run -l ko-KR`).
- **Translation Resources**: Stored in industry-standard JSON files under [`assets/locales/`](assets/locales/) compatible with modern translation management systems (Crowdin, Lokalise, Weblate, etc.).

---

## 🎨 Credits & Assets

- **Audio Assets**: Sound effect bases courtesy of [Kenney.nl](https://kenney.nl) (*UI Audio* & *Digital Audio* packs),
  licensed under [Creative Commons CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/), layered with
  custom DSP synthesized audio.
- **Fonts & Typography**: Uses [`Symbola-Subset.ttf`](assets/fonts/Symbola-Subset.ttf) supporting Latin-1, Extended
  Latin (Spanish, German, French), Cyrillic (Russian), CJK (Japanese, Simplified Chinese, Korean), and the game's
  specific UI glyphs (`☣`, `⌛`, `🛡`, `⚠`, `⭐`, `☆`, `↺`, `▶`, `⏭`, `⏮`).
