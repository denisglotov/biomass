#!/usr/bin/env bash
set -e

if [ -z "$JAVA_HOME" ]; then
  echo "✖ JAVA_HOME environment variable is not set."
  echo "  Please export JAVA_HOME in your shell (e.g. export JAVA_HOME=\"\$(/usr/libexec/java_home -v 17)\")."
  exit 1
fi

export PATH="$JAVA_HOME/bin:$PATH"

echo "=== Running Kotlin Lint Check (ktlintCheck) ==="
cd android

if [ -f "./gradlew" ]; then
  ./gradlew ktlintCheck --no-daemon
elif command -v gradle &> /dev/null; then
  gradle ktlintCheck --no-daemon
else
  echo "Gradle wrapper not found."
  exit 1
fi
