// Helper script to install Biomass APK to connected adb devices
const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const apkPath = path.join(__dirname, '..', 'android', 'app', 'build', 'outputs', 'apk', 'debug', 'app-debug.apk');

if (!fs.existsSync(apkPath)) {
  console.error(`✖ APK file not found at ${apkPath}`);
  console.error(`  Please run 'npm run build:android' first.`);
  process.exit(1);
}

try {
  const devicesOutput = execSync('adb devices', { encoding: 'utf8' });
  const lines = devicesOutput.trim().split('\n').slice(1);
  const devices = lines
    .map(line => line.split(/\s+/)[0])
    .filter(id => id && id.length > 0);

  if (devices.length === 0) {
    console.error('✖ No connected Android devices or emulators found via adb.');
    process.exit(1);
  }

  console.log(`Found ${devices.length} connected target device(s): ${devices.join(', ')}`);

  devices.forEach(deviceId => {
    console.log(`▶ Installing APK to device [${deviceId}]...`);
    try {
      execSync(`adb -s "${deviceId}" install -r "${apkPath}"`, { stdio: 'inherit' });
      console.log(`✔ Successfully installed on [${deviceId}]!`);
    } catch (e) {
      console.error(`✖ Failed to install on [${deviceId}]`);
    }
  });

} catch (err) {
  console.error('✖ ADB execution failed:', err.message);
  process.exit(1);
}
