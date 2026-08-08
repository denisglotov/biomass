#!/usr/bin/env bash
set -e

if [ -z "$JAVA_HOME" ]; then
  echo "✖ JAVA_HOME environment variable is not set."
  echo "  Please export JAVA_HOME in your shell (e.g. export JAVA_HOME=\"\$(/usr/libexec/java_home -v 17)\")."
  exit 1
fi

export PATH="$JAVA_HOME/bin:$PATH"

echo "=== Building Kotlin + Jetpack Compose Android APK (CLI) ==="
cd android

if [ -f "./gradlew" ]; then
  ./gradlew assembleDebug --no-daemon
elif command -v gradle &> /dev/null; then
  gradle assembleDebug --no-daemon
else
  echo "Gradle wrapper not initialized yet."
  exit 1
fi

echo "=== APK Built Successfully ==="
echo "APK Output: android/app/build/outputs/apk/debug/app-debug.apk"
