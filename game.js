// Biomass - Web Preview Harness & Engine Core
// Perfectly mirrors Defold Lua grid engine rules & sound synthesis

// Web Audio API Synthesizer
class SoundManager {
  constructor() {
    this.ctx = null;
    this.enabled = true;
  }

  init() {
    if (!this.ctx) {
      const AudioCtx = window.AudioContext || window.webkitAudioContext;
      if (AudioCtx) this.ctx = new AudioCtx();
    }
  }

  playWall() {
    if (!this.enabled || !this.ctx) return;
    try {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'triangle';
      osc.frequency.setValueAtTime(440, this.ctx.currentTime);
      osc.frequency.exponentialRampToValueAtTime(120, this.ctx.currentTime + 0.08);
      gain.gain.setValueAtTime(0.3, this.ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.01, this.ctx.currentTime + 0.08);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start();
      osc.stop(this.ctx.currentTime + 0.08);
    } catch(e) {}
  }

  playSpread() {
    if (!this.enabled || !this.ctx) return;
    try {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'sawtooth';
      osc.frequency.setValueAtTime(150, this.ctx.currentTime);
      osc.frequency.linearRampToValueAtTime(320, this.ctx.currentTime + 0.15);
      gain.gain.setValueAtTime(0.2, this.ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.01, this.ctx.currentTime + 0.15);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start();
      osc.stop(this.ctx.currentTime + 0.15);
    } catch(e) {}
  }

  playDieOff() {
    if (!this.enabled || !this.ctx) return;
    try {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'sine';
      osc.frequency.setValueAtTime(600, this.ctx.currentTime);
      osc.frequency.exponentialRampToValueAtTime(80, this.ctx.currentTime + 0.25);
      gain.gain.setValueAtTime(0.35, this.ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.01, this.ctx.currentTime + 0.25);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start();
      osc.stop(this.ctx.currentTime + 0.25);
    } catch(e) {}
  }

  playWin() {
    if (!this.enabled || !this.ctx) return;
    try {
      const notes = [261.63, 329.63, 392.00, 523.25];
      notes.forEach((freq, i) => {
        const osc = this.ctx.createOscillator();
        const gain = this.ctx.createGain();
        osc.type = 'sine';
        osc.frequency.setValueAtTime(freq, this.ctx.currentTime + i * 0.1);
        gain.gain.setValueAtTime(0.2, this.ctx.currentTime + i * 0.1);
        gain.gain.exponentialRampToValueAtTime(0.001, this.ctx.currentTime + i * 0.1 + 0.25);
        osc.connect(gain);
        gain.connect(this.ctx.destination);
        osc.start(this.ctx.currentTime + i * 0.1);
        osc.stop(this.ctx.currentTime + i * 0.1 + 0.25);
      });
    } catch(e) {}
  }

  playLoss() {
    if (!this.enabled || !this.ctx) return;
    try {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'sawtooth';
      osc.frequency.setValueAtTime(180, this.ctx.currentTime);
      osc.frequency.linearRampToValueAtTime(60, this.ctx.currentTime + 0.4);
      gain.gain.setValueAtTime(0.3, this.ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.01, this.ctx.currentTime + 0.4);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start();
      osc.stop(this.ctx.currentTime + 0.4);
    } catch(e) {}
  }
}

// Level Configurations (Matching scripts/level_manager.lua)
const LEVELS = [
  {
    id: 1,
    title: "Containment 101",
    description: "Learn the basics of placing barricades to trap and isolate biomass.",
    rows: 4, cols: 4,
    n_walls_per_turn: 2, n_steps_spread: 1, max_threshold: 12,
    biomass_seeds: [[1, 1]],
    initial_h_walls: [], initial_v_walls: [],
    target_turns_3star: 3, target_turns_2star: 5
  },
  {
    id: 2,
    title: "Twin Spores",
    description: "Two biomass clusters are expanding simultaneously. Contain both!",
    rows: 5, cols: 5,
    n_walls_per_turn: 2, n_steps_spread: 1, max_threshold: 18,
    biomass_seeds: [[1, 1], [3, 3]],
    initial_h_walls: [], initial_v_walls: [],
    target_turns_3star: 4, target_turns_2star: 6
  },
  {
    id: 3,
    title: "Divided Sectors",
    description: "Use pre-placed barricades to channel and seal off the bio-hazard.",
    rows: 6, cols: 6,
    n_walls_per_turn: 2, n_steps_spread: 1, max_threshold: 26,
    biomass_seeds: [[0, 2], [5, 3]],
    initial_h_walls: [[2, 1], [2, 2], [2, 3]], initial_v_walls: [[1, 2], [4, 2]],
    target_turns_3star: 5, target_turns_2star: 8
  },
  {
    id: 4,
    title: "Rapid Mutation",
    description: "WARNING: Biomass expands 2 steps per turn! Act quickly.",
    rows: 6, cols: 6,
    n_walls_per_turn: 2, n_steps_spread: 2, max_threshold: 24,
    biomass_seeds: [[2, 2], [3, 3]],
    initial_h_walls: [], initial_v_walls: [],
    target_turns_3star: 4, target_turns_2star: 7
  },
  {
    id: 5,
    title: "Corridor Siege",
    description: "A 7x7 facility under siege. Multi-flank containment required.",
    rows: 7, cols: 7,
    n_walls_per_turn: 3, n_steps_spread: 1, max_threshold: 35,
    biomass_seeds: [[1, 1], [1, 5], [5, 3]],
    initial_h_walls: [[3, 1], [3, 5]], initial_v_walls: [[1, 3], [5, 3]],
    target_turns_3star: 6, target_turns_2star: 9
  },
  {
    id: 6,
    title: "Infection Wave",
    description: "Fast-spreading biomass clusters across an 8x8 grid.",
    rows: 8, cols: 8,
    n_walls_per_turn: 3, n_steps_spread: 2, max_threshold: 45,
    biomass_seeds: [[2, 2], [2, 5], [5, 2], [5, 5]],
    initial_h_walls: [], initial_v_walls: [],
    target_turns_3star: 7, target_turns_2star: 11
  },
  {
    id: 7,
    title: "Bio-Reactor Breach",
    description: "A central reactor breach surrounded by multiple spore pockets.",
    rows: 8, cols: 8,
    n_walls_per_turn: 3, n_steps_spread: 2, max_threshold: 48,
    biomass_seeds: [[3, 3], [3, 4], [4, 3], [4, 4], [0, 0]],
    initial_h_walls: [[1, 3], [5, 3]], initial_v_walls: [[3, 1], [3, 5]],
    target_turns_3star: 8, target_turns_2star: 12
  },
  {
    id: 8,
    title: "Outbreak Zero",
    description: "The ultimate containment challenge on a 10x10 facility grid.",
    rows: 10, cols: 10,
    n_walls_per_turn: 4, n_steps_spread: 2, max_threshold: 70,
    biomass_seeds: [[1, 1], [1, 8], [8, 1], [8, 8], [4, 4], [5, 5]],
    initial_h_walls: [], initial_v_walls: [],
    target_turns_3star: 9, target_turns_2star: 14
  }
];

// Grid Engine Class (Mirroring scripts/grid.lua)
class BiomassGrid {
  constructor(config) {
    this.loadLevel(config);
  }

  loadLevel(config) {
    this.rows = config.rows;
    this.cols = config.cols;
    this.n_walls_per_turn = config.n_walls_per_turn;
    this.n_steps_spread = config.n_steps_spread;
    this.max_threshold = config.max_threshold;
    this.turn = 1;
    this.walls_placed_this_turn = 0;
    this.turn_history = [];

    // Initialize cells (0: empty, 1: biomass)
    this.cells = Array.from({ length: this.rows }, () =>
      Array.from({ length: this.cols }, () => ({ state: 0 }))
    );

    // Initialize edges (0: passable, 1: wall)
    this.h_edges = Array.from({ length: this.rows - 1 }, () =>
      Array.from({ length: this.cols }, () => 0)
    );
    this.v_edges = Array.from({ length: this.rows }, () =>
      Array.from({ length: this.cols - 1 }, () => 0)
    );

    // Initial Seeds
    config.biomass_seeds.forEach(([r, c]) => {
      if (r >= 0 && r < this.rows && c >= 0 && c < this.cols) {
        this.cells[r][c].state = 1;
      }
    });

    // Initial Walls
    config.initial_h_walls.forEach(([r, c]) => {
      if (this.h_edges[r] && this.h_edges[r][c] !== undefined) {
        this.h_edges[r][c] = 1;
      }
    });
    config.initial_v_walls.forEach(([r, c]) => {
      if (this.v_edges[r] && this.v_edges[r][c] !== undefined) {
        this.v_edges[r][c] = 1;
      }
    });
  }

  isEdgeOpen(r1, c1, r2, c2) {
    if (r1 < 0 || r1 >= this.rows || c1 < 0 || c1 >= this.cols) return false;
    if (r2 < 0 || r2 >= this.rows || c2 < 0 || c2 >= this.cols) return false;

    if (r1 === r2) {
      const minC = Math.min(c1, c2);
      if (minC >= 0 && minC <= this.cols - 2) {
        return this.v_edges[r1][minC] === 0;
      }
    } else if (c1 === c2) {
      const minR = Math.min(r1, r2);
      if (minR >= 0 && minR <= this.rows - 2) {
        return this.h_edges[minR][c1] === 0;
      }
    }
    return false;
  }

  placeWall(edgeType, r, c) {
    if (this.walls_placed_this_turn >= this.n_walls_per_turn) return false;

    if (edgeType === 'h') {
      if (r >= 0 && r < this.rows - 1 && c >= 0 && c < this.cols) {
        if (this.h_edges[r][c] === 0) {
          this.h_edges[r][c] = 1;
          this.walls_placed_this_turn++;
          this.turn_history.push({ type: 'h', r, c });
          return true;
        }
      }
    } else if (edgeType === 'v') {
      if (r >= 0 && r < this.rows && c >= 0 && c < this.cols - 1) {
        if (this.v_edges[r][c] === 0) {
          this.v_edges[r][c] = 1;
          this.walls_placed_this_turn++;
          this.turn_history.push({ type: 'v', r, c });
          return true;
        }
      }
    }
    return false;
  }

  undoWall() {
    if (this.turn_history.length === 0) return false;
    const last = this.turn_history.pop();
    if (last.type === 'h') this.h_edges[last.r][last.c] = 0;
    else if (last.type === 'v') this.v_edges[last.r][last.c] = 0;

    this.walls_placed_this_turn = Math.max(0, this.walls_placed_this_turn - 1);
    return true;
  }

  getPassableNeighbors(r, c) {
    const neighbors = [];
    const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
    dirs.forEach(([dr, dc]) => {
      const nr = r + dr, nc = c + dc;
      if (this.isEdgeOpen(r, c, nr, nc)) {
        neighbors.push({ r: nr, c: nc });
      }
    });
    return neighbors;
  }

  spreadBiomass() {
    const spreadSteps = [];
    for (let step = 0; step < this.n_steps_spread; step++) {
      const newlyInfected = [];
      const visited = new Set();

      for (let r = 0; r < this.rows; r++) {
        for (let c = 0; c < this.cols; c++) {
          if (this.cells[r][c].state === 1) {
            const nbrs = this.getPassableNeighbors(r, c);
            nbrs.forEach(n => {
              if (this.cells[n.r][n.c].state === 0) {
                const key = `${n.r}_${n.c}`;
                if (!visited.has(key)) {
                  visited.add(key);
                  newlyInfected.push({ r: n.r, c: n.c });
                }
              }
            });
          }
        }
      }

      if (newlyInfected.length === 0) break;
      newlyInfected.forEach(cell => {
        this.cells[cell.r][cell.c].state = 1;
      });
      spreadSteps.push(newlyInfected);
    }
    return spreadSteps;
  }

  evaluateIsolation() {
    const diedCells = [];
    const visited = new Set();

    for (let r = 0; r < this.rows; r++) {
      for (let c = 0; c < this.cols; c++) {
        const key = `${r}_${c}`;
        if (this.cells[r][c].state === 1 && !visited.has(key)) {
          const component = [];
          const queue = [{ r, c }];
          visited.add(key);
          let reachableEmptyCells = 0;

          let head = 0;
          while (head < queue.length) {
            const curr = queue[head++];
            component.push(curr);

            const nbrs = this.getPassableNeighbors(curr.r, curr.c);
            nbrs.forEach(n => {
              if (this.cells[n.r][n.c].state === 0) {
                reachableEmptyCells++;
              } else if (this.cells[n.r][n.c].state === 1) {
                const nkey = `${n.r}_${n.c}`;
                if (!visited.has(nkey)) {
                  visited.add(nkey);
                  queue.push({ r: n.r, c: n.c });
                }
              }
            });
          }

          // Sealed Enclosure Rule: if no empty cell reachable anywhere, component dies!
          if (reachableEmptyCells === 0) {
            component.forEach(cell => {
              this.cells[cell.r][cell.c].state = 0;
              diedCells.push({ r: cell.r, c: cell.c });
            });
          }
        }
      }
    }
    return diedCells;
  }

  getBiomassCount() {
    let count = 0;
    for (let r = 0; r < this.rows; r++) {
      for (let c = 0; c < this.cols; c++) {
        if (this.cells[r][c].state === 1) count++;
      }
    }
    return count;
  }

  getOpenEdgesCount() {
    let count = 0;
    for (let r = 0; r < this.rows - 1; r++) {
      for (let c = 0; c < this.cols; c++) {
        if (this.h_edges[r][c] === 0) count++;
      }
    }
    for (let r = 0; r < this.rows; r++) {
      for (let c = 0; c < this.cols - 1; c++) {
        if (this.v_edges[r][c] === 0) count++;
      }
    }
    return count;
  }

  checkStatus() {
    const bCount = this.getBiomassCount();
    if (bCount === 0) return "win";
    if (bCount >= this.max_threshold) return "loss";
    if (this.getOpenEdgesCount() === 0 && bCount > 0) return "loss";
    return "ongoing";
  }

  endTurn() {
    const spreadSteps = this.spreadBiomass();
    const diedCells = this.evaluateIsolation();
    this.turn++;
    this.walls_placed_this_turn = 0;
    this.turn_history = [];
    const status = this.checkStatus();
    return { spreadSteps, diedCells, status };
  }
}

// UI Controller & Render Engine
class GameApp {
  constructor() {
    this.canvas = document.getElementById('gameCanvas');
    this.ctx = this.canvas.getContext('2d');
    this.sound = new SoundManager();

    this.currentLevelIdx = 0;
    this.grid = new BiomassGrid(LEVELS[this.currentLevelIdx]);
    this.animSpeed = 1; // 1x, 2x, 5x
    this.hoverEdge = null; // { type: 'h'|'v', r, c }
    this.isAnimating = false;

    this.time = 0;
    this.initUI();
    this.resizeCanvas();
    this.render();

    // Pulse animation loop
    requestAnimationFrame(this.loop.bind(this));
  }

  initUI() {
    // Canvas interaction
    this.canvas.addEventListener('mousemove', (e) => this.handleMouseMove(e));
    this.canvas.addEventListener('mouseleave', () => { this.hoverEdge = null; });
    this.canvas.addEventListener('click', (e) => this.handleClick(e));

    // Level selector
    const levelSelect = document.getElementById('levelSelect');
    levelSelect.addEventListener('change', (e) => {
      this.currentLevelIdx = parseInt(e.target.value) - 1;
      this.loadLevel();
    });

    // Buttons
    document.getElementById('undoBtn').addEventListener('click', () => {
      this.sound.init();
      if (this.grid.undoWall()) {
        this.sound.playWall();
        this.updateStats();
      }
    });

    document.getElementById('resetBtn').addEventListener('click', () => {
      this.sound.init();
      this.loadLevel();
    });

    document.getElementById('endTurnBtn').addEventListener('click', () => {
      this.sound.init();
      this.triggerEndTurn();
    });

    // Speed toggle
    document.querySelectorAll('.speed-btn').forEach(btn => {
      btn.addEventListener('click', (_e) => {
        document.querySelectorAll('.speed-btn').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        this.animSpeed = parseInt(btn.dataset.speed);
      });
    });

    // Modal buttons
    document.getElementById('modalRetryBtn').addEventListener('click', () => {
      this.hideModal();
      this.loadLevel();
    });

    document.getElementById('modalNextBtn').addEventListener('click', () => {
      this.hideModal();
      if (this.currentLevelIdx < LEVELS.length - 1) {
        this.currentLevelIdx++;
        document.getElementById('levelSelect').value = (this.currentLevelIdx + 1).toString();
      }
      this.loadLevel();
    });

    this.updateStats();
  }

  loadLevel() {
    const config = LEVELS[this.currentLevelIdx];
    this.grid.loadLevel(config);
    this.isAnimating = false;
    this.hoverEdge = null;

    document.getElementById('levelTitle').textContent = `Level ${config.id}: ${config.title}`;
    document.getElementById('levelDesc').textContent = config.description;
    document.getElementById('spreadRateText').textContent = `${config.n_steps_spread} step${config.n_steps_spread > 1 ? 's' : ''}/turn`;

    this.resizeCanvas();
    this.updateStats();
  }

  resizeCanvas() {
    const rows = this.grid.rows;
    const cols = this.grid.cols;
    const maxDim = Math.max(rows, cols);
    const cellSize = Math.min(540 / maxDim, 80);

    this.cellSize = cellSize;
    this.padding = 35;
    this.width = cols * cellSize + this.padding * 2;
    this.height = rows * cellSize + this.padding * 2;

    this.canvas.width = this.width;
    this.canvas.height = this.height;
  }

  getClosestEdge(clientX, clientY) {
    const rect = this.canvas.getBoundingClientRect();
    const scaleX = this.canvas.width / rect.width;
    const scaleY = this.canvas.height / rect.height;

    const mx = (clientX - rect.left) * scaleX;
    const my = (clientY - rect.top) * scaleY;

    const gx = mx - this.padding;
    const gy = my - this.padding;

    const c = Math.floor(gx / this.cellSize);
    const r = Math.floor(gy / this.cellSize);

    if (r < 0 || r >= this.grid.rows || c < 0 || c >= this.grid.cols) {
      return null;
    }

    const distTop = gy - r * this.cellSize;
    const distBottom = (r + 1) * this.cellSize - gy;
    const distLeft = gx - c * this.cellSize;
    const distRight = (c + 1) * this.cellSize - gx;

    const minDist = Math.min(distTop, distBottom, distLeft, distRight);

    let candidates = [];
    if (minDist === distTop && r > 0) candidates.push({ type: 'h', r: r - 1, c: c });
    if (minDist === distBottom && r < this.grid.rows - 1) candidates.push({ type: 'h', r: r, c: c });
    if (minDist === distLeft && c > 0) candidates.push({ type: 'v', r: r, c: c - 1 });
    if (minDist === distRight && c < this.grid.cols - 1) candidates.push({ type: 'v', r: r, c: c });

    for (const cand of candidates) {
      if (cand.type === 'h' && this.grid.h_edges[cand.r][cand.c] === 0) return cand;
      if (cand.type === 'v' && this.grid.v_edges[cand.r][cand.c] === 0) return cand;
    }

    return null;
  }

  handleMouseMove(e) {
    if (this.isAnimating) return;
    this.hoverEdge = this.getClosestEdge(e.clientX, e.clientY);
  }

  handleClick(e) {
    if (this.isAnimating) return;
    this.sound.init();

    const targetEdge = this.getClosestEdge(e.clientX, e.clientY) || this.hoverEdge;

    if (targetEdge) {
      const placed = this.grid.placeWall(targetEdge.type, targetEdge.r, targetEdge.c);
      if (placed) {
        this.sound.playWall();
        this.hoverEdge = null;
        this.updateStats();

        // Auto-end turn if player reached max walls
        if (this.grid.walls_placed_this_turn >= this.grid.n_walls_per_turn) {
          setTimeout(() => this.triggerEndTurn(), 250);
        }
      }
    }
  }

  async triggerEndTurn() {
    if (this.isAnimating) return;
    this.isAnimating = true;
    this.hoverEdge = null;

    // Execute Spread step by step with animations
    for (let step = 0; step < this.grid.n_steps_spread; step++) {
      const newlyInfected = [];
      const visited = new Set();

      for (let r = 0; r < this.grid.rows; r++) {
        for (let c = 0; c < this.grid.cols; c++) {
          if (this.grid.cells[r][c].state === 1) {
            const nbrs = this.grid.getPassableNeighbors(r, c);
            nbrs.forEach(n => {
              if (this.grid.cells[n.r][n.c].state === 0) {
                const key = `${n.r}_${n.c}`;
                if (!visited.has(key)) {
                  visited.add(key);
                  newlyInfected.push({ r: n.r, c: n.c });
                }
              }
            });
          }
        }
      }

      if (newlyInfected.length > 0) {
        newlyInfected.forEach(cell => {
          this.grid.cells[cell.r][cell.c].state = 1;
        });
        this.sound.playSpread();
        this.updateStats();
        if (this.animSpeed < 5) {
          await new Promise(res => setTimeout(res, 400 / this.animSpeed));
        }
      } else {
        break;
      }
    }

    // Execute Isolation Die-off
    const diedCells = this.grid.evaluateIsolation();
    if (diedCells.length > 0) {
      this.sound.playDieOff();
      this.updateStats();
      if (this.animSpeed < 5) {
        await new Promise(res => setTimeout(res, 500 / this.animSpeed));
      }
    }

    this.grid.turn++;
    this.grid.walls_placed_this_turn = 0;
    this.grid.turn_history = [];

    const status = this.grid.checkStatus();

    this.isAnimating = false;
    this.updateStats();

    if (status === 'win') {
      this.sound.playWin();
      this.showWinModal();
    } else if (status === 'loss') {
      this.sound.playLoss();
      this.showLossModal();
    }
  }

  updateStats() {
    document.getElementById('turnCounter').textContent = this.grid.turn;
    const wallsLeft = this.grid.n_walls_per_turn - this.grid.walls_placed_this_turn;
    document.getElementById('wallsLeftCounter').textContent = `${wallsLeft} / ${this.grid.n_walls_per_turn}`;
    
    const bioCount = this.grid.getBiomassCount();
    const bioElem = document.getElementById('biomassCounter');
    bioElem.textContent = bioCount;
    if (bioCount >= this.grid.max_threshold * 0.75) {
      bioElem.classList.add('danger');
    } else {
      bioElem.classList.remove('danger');
    }

    document.getElementById('maxThresholdCounter').textContent = this.grid.max_threshold;
  }

  showWinModal() {
    const config = LEVELS[this.currentLevelIdx];
    const turns = this.grid.turn - 1;
    let stars = 1;
    if (turns <= config.target_turns_3star) stars = 3;
    else if (turns <= config.target_turns_2star) stars = 2;

    const modal = document.getElementById('modalOverlay');
    const card = document.getElementById('modalCard');
    card.className = 'modal-card win-modal';

    document.getElementById('modalTitle').textContent = 'CONTAINMENT COMPLETE';
    document.getElementById('modalBody').innerHTML = 
      `Sector <strong>${config.title}</strong> successfully purged of all biomass hazards in <strong>${turns}</strong> turn(s)!`;

    const starContainer = document.getElementById('starContainer');
    starContainer.innerHTML = '';
    for (let i = 1; i <= 3; i++) {
      const span = document.createElement('span');
      span.className = `star ${i <= stars ? 'active' : ''}`;
      span.textContent = '★';
      starContainer.appendChild(span);
    }

    modal.classList.add('active');
  }

  showLossModal() {
    const modal = document.getElementById('modalOverlay');
    const card = document.getElementById('modalCard');
    card.className = 'modal-card loss-modal';

    document.getElementById('modalTitle').textContent = 'CONTAINMENT BREACH';
    document.getElementById('modalBody').textContent = 
      'Biomass has breached safety limits or no barricade moves remain. Re-evaluate your strategy!';

    const starContainer = document.getElementById('starContainer');
    starContainer.innerHTML = '<span class="star">★</span><span class="star">★</span><span class="star">★</span>';

    modal.classList.add('active');
  }

  hideModal() {
    document.getElementById('modalOverlay').classList.remove('active');
  }

  loop() {
    this.time += 0.05;
    this.render();
    requestAnimationFrame(this.loop.bind(this));
  }

  render() {
    this.ctx.clearRect(0, 0, this.width, this.height);

    const rows = this.grid.rows;
    const cols = this.grid.cols;
    const cs = this.cellSize;
    const pad = this.padding;

    // Draw Grid Cells
    for (let r = 0; r < rows; r++) {
      for (let c = 0; c < cols; c++) {
        const x = pad + c * cs;
        const y = pad + r * cs;

        const isBiomass = this.grid.cells[r][c].state === 1;

        // Background cell floor - high contrast slate blue
        if (isBiomass) {
          this.ctx.fillStyle = 'rgba(20, 50, 35, 0.95)';
        } else {
          this.ctx.fillStyle = (r + c) % 2 === 0 ? '#152238' : '#1b2c48';
        }
        this.ctx.fillRect(x, y, cs, cs);

        // High contrast grid cell borders
        this.ctx.strokeStyle = isBiomass ? 'rgba(57, 255, 20, 0.6)' : 'rgba(0, 229, 255, 0.35)';
        this.ctx.lineWidth = 2;
        this.ctx.strokeRect(x, y, cs, cs);

        // Render Biomass
        if (isBiomass) {
          try {
            const cx = x + cs / 2;
            const cy = y + cs / 2;
            const t = this.time || 0;
            const pulse = Math.sin(t * 4 + r + c) * 3;
            const radius = Math.max(8, cs * 0.32 + pulse);
            const outerR = Math.max(radius + 2, radius * 1.35);

            // Glowing Outer Bio Aura
            const grad = this.ctx.createRadialGradient(cx, cy, 2, cx, cy, outerR);
            grad.addColorStop(0, 'rgba(57, 255, 20, 0.9)');
            grad.addColorStop(0.5, 'rgba(0, 255, 136, 0.5)');
            grad.addColorStop(1, 'rgba(57, 255, 20, 0)');
            
            this.ctx.fillStyle = grad;
            this.ctx.beginPath();
            this.ctx.arc(cx, cy, outerR, 0, Math.PI * 2);
            this.ctx.fill();

            // Main Bio Nucleus
            this.ctx.fillStyle = '#39ff14';
            this.ctx.beginPath();
            this.ctx.arc(cx, cy, radius, 0, Math.PI * 2);
            this.ctx.fill();

            // Orbiting Spore Blobs
            for (let i = 0; i < 3; i++) {
              const angle = t * 2 + (i * Math.PI * 2 / 3);
              const orbitR = radius * 0.5;
              const ox = cx + Math.cos(angle) * orbitR;
              const oy = cy + Math.sin(angle) * orbitR;
              this.ctx.fillStyle = '#00ff88';
              this.ctx.beginPath();
              this.ctx.arc(ox, oy, radius * 0.28, 0, Math.PI * 2);
              this.ctx.fill();
            }

            // Inner Bright White Core
            this.ctx.fillStyle = '#ffffff';
            this.ctx.beginPath();
            this.ctx.arc(cx - radius * 0.15, cy - radius * 0.15, radius * 0.28, 0, Math.PI * 2);
            this.ctx.fill();
          } catch(e) {
            console.error("Error drawing biomass cell:", e);
          }
        }
      }
    }

    // Outer Facility Boundary Wall
    this.ctx.strokeStyle = '#00e5ff';
    this.ctx.lineWidth = 6;
    this.ctx.shadowColor = '#00e5ff';
    this.ctx.shadowBlur = 10;
    this.ctx.strokeRect(pad, pad, cols * cs, rows * cs);
    this.ctx.shadowBlur = 0;

    // Horizontal Barricade Walls
    for (let r = 0; r < rows - 1; r++) {
      for (let c = 0; c < cols; c++) {
        if (this.grid.h_edges[r][c] === 1) {
          const x1 = pad + c * cs;
          const y = pad + (r + 1) * cs;
          const x2 = pad + (c + 1) * cs;

          this.drawWallLine(x1, y, x2, y);
        }
      }
    }

    // Vertical Barricade Walls
    for (let r = 0; r < rows; r++) {
      for (let c = 0; c < cols - 1; c++) {
        if (this.grid.v_edges[r][c] === 1) {
          const x = pad + (c + 1) * cs;
          const y1 = pad + r * cs;
          const y2 = pad + (r + 1) * cs;

          this.drawWallLine(x, y1, x, y2);
        }
      }
    }

    // Hover Edge Preview - Vivid Electric Yellow/Cyan Glow
    if (this.hoverEdge) {
      this.ctx.save();
      const pulseGlow = 15 + Math.sin(this.time * 6) * 5;
      this.ctx.strokeStyle = '#ffea00';
      this.ctx.lineWidth = 8;
      this.ctx.shadowColor = '#ffea00';
      this.ctx.shadowBlur = pulseGlow;

      if (this.hoverEdge.type === 'h') {
        const x1 = pad + this.hoverEdge.c * cs;
        const y = pad + (this.hoverEdge.r + 1) * cs;
        const x2 = pad + (this.hoverEdge.c + 1) * cs;
        this.ctx.beginPath();
        this.ctx.moveTo(x1, y);
        this.ctx.lineTo(x2, y);
        this.ctx.stroke();
      } else {
        const x = pad + (this.hoverEdge.c + 1) * cs;
        const y1 = pad + this.hoverEdge.r * cs;
        const y2 = pad + (this.hoverEdge.r + 1) * cs;
        this.ctx.beginPath();
        this.ctx.moveTo(x, y1);
        this.ctx.lineTo(x, y2);
        this.ctx.stroke();
      }
      this.ctx.restore();
    }
  }

  drawWallLine(x1, y1, x2, y2) {
    this.ctx.save();
    
    // Wall Outer Glow
    this.ctx.strokeStyle = 'rgba(0, 229, 255, 0.8)';
    this.ctx.lineWidth = 12;
    this.ctx.shadowColor = '#00e5ff';
    this.ctx.shadowBlur = 18;
    this.ctx.beginPath();
    this.ctx.moveTo(x1, y1);
    this.ctx.lineTo(x2, y2);
    this.ctx.stroke();

    // Core White Line
    this.ctx.strokeStyle = '#ffffff';
    this.ctx.lineWidth = 4;
    this.ctx.shadowBlur = 0;
    this.ctx.beginPath();
    this.ctx.moveTo(x1, y1);
    this.ctx.lineTo(x2, y2);
    this.ctx.stroke();

    // End Nodes
    this.ctx.fillStyle = '#00e5ff';
    this.ctx.beginPath();
    this.ctx.arc(x1, y1, 5, 0, Math.PI * 2);
    this.ctx.arc(x2, y2, 5, 0, Math.PI * 2);
    this.ctx.fill();

    this.ctx.restore();
  }
}

// Start App when page loads
window.addEventListener('DOMContentLoaded', () => {
  window.app = new GameApp();
});
