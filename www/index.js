import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 7;
var style = getComputedStyle(document.body);
const ALIVE_COLOR = style.getPropertyValue('--cell-color');
const DEAD_COLOR = style.getPropertyValue('--bg-left-color');

var height = Math.floor(document.documentElement.clientHeight / 8) + 2
var width = Math.floor(document.documentElement.clientWidth / 16)
var universe = Universe.new(width, height);

var canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height;
canvas.width = (CELL_SIZE + 1) * width;

const ctx = canvas.getContext('2d');

const resizeUniverse = () => {
  height = Math.floor(document.documentElement.clientHeight / 8) + 2
  width = Math.floor(document.documentElement.clientWidth / 16)
  universe.resize(width, height);
  canvas.height = (CELL_SIZE + 1) * height;
  canvas.width = (CELL_SIZE + 1) * width;
}
window.addEventListener("resize", resizeUniverse);

const renderLoop = () => {
  universe.tick();

  drawCells();

  requestAnimationFrame(renderLoop);
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  var cellsPtr = universe.cells();
  var cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  ctx.fillStyle = ALIVE_COLOR;
  for (let row  = 0; row < height - 2; row++) {
    for (let col = 0; col < width; col++) {
      if (cells[row * width + col] !== Cell.Alive) {
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

  ctx.fillStyle = DEAD_COLOR;
  for (let row  = 0; row < height - 2; row++) {
    for (let col = 0; col < width; col++) {
      if (cells[row * width + col] !== Cell.Dead) {
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

drawCells();
requestAnimationFrame(renderLoop);
