#!/usr/bin/env bash
set -e

echo "=== Building Kotlin + Jetpack Compose Android APK (CLI) ==="
cd android

if [ -f "./gradlew" ]; then
  ./gradlew assembleDebug
elif command -v gradle &> /dev/null; then
  gradle assembleDebug
else
  echo "Gradle wrapper not initialized yet."
  exit 1
fi

echo "=== APK Built Successfully ==="
echo "APK Output: android/app/build/outputs/apk/debug/app-debug.apk"
