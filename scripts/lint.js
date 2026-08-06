// Comprehensive Linter Runner for Biomass Project
// Lints JavaScript (game.js) via ESLint and Lua scripts via luacheck / luac static analysis.

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const GREEN = '\x1b[32m';
const RED = '\x1b[31m';
const CYAN = '\x1b[36m';
const YELLOW = '\x1b[33m';
const RESET = '\x1b[0m';

let hasErrors = false;

console.log(`${CYAN}==========================================${RESET}`);
console.log(`${CYAN}   BIOMASS PROJECT CODE QUALITY LINTER    ${RESET}`);
console.log(`${CYAN}==========================================${RESET}\n`);

// Helper: Recursively locate Lua & script files
function findLuaFiles(dir) {
  let results = [];
  if (!fs.existsSync(dir)) return results;
  const list = fs.readdirSync(dir);
  list.forEach(file => {
    const filePath = path.join(dir, file);
    const stat = fs.statSync(filePath);
    if (stat && stat.isDirectory()) {
      results = results.concat(findLuaFiles(filePath));
    } else if (file.endsWith('.lua') || file.endsWith('.script')) {
      results.push(filePath);
    }
  });
  return results;
}

// 1. Lint JavaScript Files
console.log(`${YELLOW}▶ Linting JavaScript (game.js)...${RESET}`);
try {
  execSync('npx eslint game.js scripts/lint.js', { encoding: 'utf8' });
  console.log(`${GREEN}✔ JavaScript linting passed cleanly!${RESET}\n`);
} catch (err) {
  hasErrors = true;
  console.error(`${RED}✖ ESLint found issues in JavaScript files:${RESET}`);
  console.error(err.stdout || err.message);
  console.log('');
}

// 2. Lint Lua Scripts (Static Analysis via luacheck & syntax check via luac)
console.log(`${YELLOW}▶ Linting Lua scripts (scripts/*.lua, main/*.script)...${RESET}`);

let hasLuacheck = false;
try {
  execSync('which luacheck', { stdio: 'ignore' });
  hasLuacheck = true;
} catch (e) {}

if (hasLuacheck) {
  try {
    execSync('luacheck scripts/ main/', { stdio: 'inherit' });
    console.log(`${GREEN}✔ Luacheck static analysis passed cleanly!${RESET}\n`);
  } catch (err) {
    hasErrors = true;
    console.error(`${RED}✖ Luacheck found issues in Lua scripts.${RESET}\n`);
  }
} else {
  // Fallback: luac syntax parser
  const luaFiles = [...findLuaFiles('./scripts'), ...findLuaFiles('./main')];
  let luaSuccessCount = 0;

  luaFiles.forEach(file => {
    try {
      execSync(`luac -p "${file}"`, { stdio: 'pipe' });
      console.log(`  ${GREEN}✔ ${file}${RESET}`);
      luaSuccessCount++;
    } catch (err) {
      hasErrors = true;
      console.error(`  ${RED}✖ Syntax error in ${file}:${RESET}`);
      console.error(err.stderr ? err.stderr.toString() : err.message);
    }
  });
  console.log(`\n${GREEN}Checked ${luaSuccessCount}/${luaFiles.length} Lua files successfully.${RESET}\n`);
}

if (hasErrors) {
  console.error(`${RED}✖ Linting failed. Please fix the reported errors above.${RESET}`);
  process.exit(1);
} else {
  console.log(`${GREEN}🎉 All JavaScript and Lua code linted cleanly with zero errors!${RESET}`);
}
