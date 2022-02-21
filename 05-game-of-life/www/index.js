import { memory } from "game-of-life/game_of_life_bg";
import { Universe, Cell } from "game-of-life";

// const pre = document.getElementById("game-of-life-canvas");
// const universe = Universe.new();

// const renderLoop = () => {
//     pre.textContent = universe.render();
//     universe.tick();

//     requestAnimationFrame(renderLoop);
//   };

//   requestAnimationFrame(renderLoop);

const CELL_SIZE = 5; // px
const GRID_COLOR = "#ccc";
const DEAD_COLOR = "#fff";
const ALIVE_COLOR = "#000";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

let animationId = null;

const renderLoop = () => {
  //   debugger;
  fps.render();

  drawGrid();
  drawCells();
  // making time run faster
  //   for (let i = 0; i < 9; i++) {
  universe.tick();
  //   }
  animationId = requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  ctx.beginPath();
  //   for (let row = 0; row < height; row++) {
  //     for (let col = 0; col < width; col++) {
  //       const idx = getIndex(row, col);
  // fillStyle property is set once for every cell in the universe, on every animation frame
  // 40% of our time is spent in this setter!
  //   ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
  //   ctx.fillRect(
  //     col * (CELL_SIZE + 1) + 1,
  //     row * (CELL_SIZE + 1) + 1,
  //     CELL_SIZE,
  //     CELL_SIZE
  //   );

  //     }
  //   }

  // Alive cells
  ctx.fillStyle = ALIVE_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      if (cells[idx] !== Cell.Alive) {
        continue;
      }

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  // Dead cells
  ctx.fillStyle = DEAD_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      if (cells[idx] !== Cell.Dead) {
        continue;
      }

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

// Pausing and Resuming the Game
const isPaused = () => animationId === null;

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", (event) =>
  isPaused() ? play() : pause()
);

// Toggling a Cell's State on "click" Events
canvas.addEventListener("click", (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1), height - 1));
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1), width - 1));

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
});

// Creating a Frames Per Second Timer
const fps = new (class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = (1 / delta) * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    // Optimize until you get 60 frames per second
    this.fps.textContent = `
Frames per second:
Latest = ${Math.round(fps)}
Avg of last 100 = ${Math.round(mean)} 
Min of last 100 = ${Math.round(min)}
Max of last 100 = ${Math.round(max)}
    `;
  }
})();

// Init
drawGrid();
drawCells();
// requestAnimationFrame(renderLoop);
play();
