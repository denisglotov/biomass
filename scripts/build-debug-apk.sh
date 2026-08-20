#!/usr/bin/env bash
set -euo pipefail

# Build a debug APK with an isolated application ID (org.dymka.debug.biomass) so it
# can coexist with the Google Play release version on the same device.
# Note: cargo-quad-apk uses the last segment of package_name ("biomass") as the native
# library name to load in MainActivity.java (System.loadLibrary("biomass")), which must match
# the crate name libbiomass.so.

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CARGO_TOML="${PROJECT_ROOT}/Cargo.toml"

# Backup and restore Cargo.toml on exit (success or failure)
cp "$CARGO_TOML" "${CARGO_TOML}.bak"
trap 'mv "${CARGO_TOML}.bak" "$CARGO_TOML"' EXIT

# Patch package_name and label for the debug variant
sed -i '' \
  -e 's/^package_name = "org\.dymka\.biomass"$/package_name = "org.dymka.debug.biomass"/' \
  -e 's/^label = "Biomass"$/label = "Biomass (Debug)"/' \
  "$CARGO_TOML"

echo "==> Building debug APK (org.dymka.debug.biomass)..."
cargo quad-apk build

BUILD_APK_PATH="${PROJECT_ROOT}/target/android-artifacts/debug/apk/biomass.apk"
FINAL_APK_PATH="${PROJECT_ROOT}/target/android-artifacts/debug/apk/biomass-debug.apk"
if [ -f "$BUILD_APK_PATH" ]; then
  mv "$BUILD_APK_PATH" "$FINAL_APK_PATH"
  echo "==> Debug APK ready: $FINAL_APK_PATH"
  echo "    Install with: adb install -r $FINAL_APK_PATH"
elif [ -f "$FINAL_APK_PATH" ]; then
  echo "==> Debug APK ready: $FINAL_APK_PATH"
  echo "    Install with: adb install -r $FINAL_APK_PATH"
else
  echo "==> Build finished. Check target/android-artifacts/debug/ for output."
fi
