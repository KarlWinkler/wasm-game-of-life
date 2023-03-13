import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

let cell_size = 5; // px
let grid_color = "#36002d";
let dead_color = "#36002d";
let alive_color = "#4287f5";

const universe = Universe.new(128, 64);
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (cell_size + 1) * height + 1;
canvas.width = (cell_size + 1) * width + 1;

const ctx = canvas.getContext('2d');

const playPauseButton = document.getElementById("play-pause");
let isPaused = false;

const resetButton = document.getElementById("reset");

const clearButton = document.getElementById("clear");

const nextButton = document.getElementById("next-step");

const zoom = document.getElementById("zoom");

const backgroundColorPicker = document.getElementById("background-color");
const forgroundColorPicker = document.getElementById("foreground-color");
const gridColorPicker = document.getElementById("grid-color");

const renderLoop = async () => {
  await new Promise(r => setTimeout(r, 50));

  if (!isPaused) {
    universe.tick();

    drawGrid();
    drawCells();
  }

  requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = grid_color;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (cell_size + 1) + 1, 0);
    ctx.lineTo(i * (cell_size + 1) + 1, (cell_size + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    for (let k = 0; k < 8; k++) {
      ctx.moveTo(0, (j + k) * (cell_size + 1) + 1);
      ctx.lineTo((cell_size + 1) * width + 1, (j + k) * (cell_size + 1) + 1);
    }
  }

  ctx.stroke();
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  // console.log(cells)
  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = bitIsSet(idx, cells)
        ? dead_color
        : alive_color;

      ctx.fillRect(
        col * (cell_size + 1) + 1,
        row * (cell_size + 1) + 1,
        cell_size,
        cell_size
      );
    }
  }

  ctx.stroke();
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n % 8);
  return (arr[byte] & mask) !== mask;
};

const getIndex = (row, column) => {
  return row * width + column;
};

playPauseButton.addEventListener("click", event => {
  playPauseButton.innerHTML = isPaused ? "Pause" : "Play";
  isPaused = !isPaused;
});

resetButton.addEventListener("click", event => {
  universe.reset();
  drawGrid();
  drawCells();
});

clearButton.addEventListener("click", event => {
  universe.clear();
  drawGrid();
  drawCells();
});

nextButton.addEventListener("click", event => {
  universe.tick();
  drawGrid();
  drawCells();
});

zoom.addEventListener("change", event => {
  cell_size = parseInt(zoom.value);
  canvas.height = (cell_size + 1) * height + 1;
  canvas.width = (cell_size + 1) * width + 1;
  drawGrid();
  drawCells();
});

backgroundColorPicker.addEventListener("change", event => {
  dead_color = backgroundColorPicker.value;
  drawGrid();
  drawCells();
});

forgroundColorPicker.addEventListener("change", event => {
  alive_color = forgroundColorPicker.value;
  drawGrid();
  drawCells();
});

gridColorPicker.addEventListener("change", event => {
  grid_color = gridColorPicker.value;
  drawGrid();
  drawCells();
});

canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (cell_size + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (cell_size + 1)), width - 1);

  if (window.event.ctrlKey) {
    //ctrl was held down during the click
    universe.set_glider(row, col);
  }
  else if (window.event.shiftKey) {
    //shift was held down during the click
    universe.set_pulsar(row, col);
  }
  else {
    universe.toggle_cell(row, col);
  }

  drawGrid();
  drawCells();
});

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);