#!/usr/bin/env bash
set -euo pipefail

# Ensure ANDROID_HOME is set
if [ -z "${ANDROID_HOME:-}" ]; then
  export ANDROID_HOME="/opt/homebrew/share/android-commandlinetools"
fi

# Locate build-tools and android.jar
BUILD_TOOLS_DIR="${ANDROID_HOME}/build-tools/35.0.0"
if [ ! -d "$BUILD_TOOLS_DIR" ]; then
  BUILD_TOOLS_DIR=$(ls -d "${ANDROID_HOME}/build-tools/"* 2>/dev/null | tail -n 1)
fi

AAPT2="${BUILD_TOOLS_DIR}/aapt2"
ANDROID_JAR="${ANDROID_HOME}/platforms/android-35/android.jar"
if [ ! -f "$ANDROID_JAR" ]; then
  ANDROID_JAR=$(ls "${ANDROID_HOME}/platforms/android-"*/android.jar 2>/dev/null | tail -n 1)
fi

if ! command -v bundletool &>/dev/null; then
  echo "Error: bundletool is required but not found in PATH." >&2
  echo "Install it via: brew install bundletool" >&2
  exit 1
fi

BIN_DIR="target/android-artifacts/release/bin/biomass"
APK_OUT_DIR="target/android-artifacts/release/apk"
TMP_DIR="target/android-artifacts/release/aab_tmp"

if [ ! -d "$BIN_DIR" ]; then
  echo "Error: Android build output directory '$BIN_DIR' does not exist." >&2
  echo "Please run 'cargo quad-apk build --release' first." >&2
  exit 1
fi

echo "==> Packaging Android App Bundle (.aab)..."
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR/bundle_root/manifest" "$TMP_DIR/bundle_root/dex" "$APK_OUT_DIR"

# 1. Compile resources into proto format for AAB
"$AAPT2" compile --dir res -o "$TMP_DIR/compiled_res.zip"
"$AAPT2" link --proto-format -o "$TMP_DIR/base_linked.apk" \
  -I "$ANDROID_JAR" \
  --manifest "$BIN_DIR/AndroidManifest.xml" \
  "$TMP_DIR/compiled_res.zip" -A assets

# 2. Unpack proto-linked APK and structure bundle module directory
unzip -q "$TMP_DIR/base_linked.apk" -d "$TMP_DIR/bundle_root/"
mv "$TMP_DIR/bundle_root/AndroidManifest.xml" "$TMP_DIR/bundle_root/manifest/"
cp "$BIN_DIR/classes.dex" "$TMP_DIR/bundle_root/dex/"
cp -R "$BIN_DIR/lib" "$TMP_DIR/bundle_root/"

# 3. Zip module structure and build final AAB using bundletool
(cd "$TMP_DIR/bundle_root" && zip -q -r ../base.zip .)
rm -f "$APK_OUT_DIR/biomass.aab"
bundletool build-bundle --overwrite --modules="$TMP_DIR/base.zip" --output="$APK_OUT_DIR/biomass.aab"

# Cleanup temporary files
rm -rf "$TMP_DIR"

echo "==> Successfully created AAB: $APK_OUT_DIR/biomass.aab"
