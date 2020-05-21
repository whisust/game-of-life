import {Cell, State} from "game-of-life";
import {memory} from "game-of-life/game_of_life_bg";

// Constants
const CELL_SIZE = 5;
const MARGIN = 1;
const CELL_WITH_MARGIN = CELL_SIZE + MARGIN;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Run variables
const game = State.new(120, 150);
const infos = document.getElementById('infos');
const canvas = document.getElementById('game-of-life-canvas');
const playPauseButton = document.getElementById('play-pause');
canvas.height = CELL_WITH_MARGIN * game.height + 1;
canvas.width = CELL_WITH_MARGIN * game.width + 1;
const ctx = canvas.getContext('2d');

// Values for the animation function
let fpsInterval, startTime, now, then, elapsed;
let requestId;

// initialize the timer variables and start the animation
function startAnimating(fps) {
    fpsInterval = 1000 / fps;
    then = Date.now();
    startTime = then;
    animate();
}

const isPaused = () => {
    return requestId === null;
}

const getIndex = (row, column) => {
    return row * game.width + column;
}

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // vertical lines
    for (let i = 0; i <= game.width; i++) {
        ctx.moveTo(i * CELL_WITH_MARGIN + 1, 0);
        ctx.lineTo(i * CELL_WITH_MARGIN + 1, i * CELL_WITH_MARGIN * game.height + 1);
    }

    // horizontal lines
    for (let j = 0; j <= game.width; j++) {
        ctx.moveTo(0, j * CELL_WITH_MARGIN + 1);
        ctx.lineTo(game.width * CELL_WITH_MARGIN + 1, j * CELL_WITH_MARGIN + 1);
    }

    ctx.stroke();
}

const drawCells = () => {
    const cellsPointer = game.cells();
    const cells = new Uint8Array(memory.buffer, cellsPointer, game.width * game.height);

    ctx.beginPath();

    for (let row = 0; row < game.height; row++) {
        for (let col = 0; col < game.width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Alive ? ALIVE_COLOR : DEAD_COLOR;
            ctx.fillRect(
                col * CELL_WITH_MARGIN + 1,
                row * CELL_WITH_MARGIN + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    ctx.stroke();
}
const render = () => {
    infos.textContent = `Grid ${game.width} x ${game.height} - Generation ${game.generation}`;
    drawGrid();
    drawCells();
    game.next();
}

function animate() {

    // request another frame

    requestId = requestAnimationFrame(animate);

    // calc elapsed time since last loop

    now = Date.now();
    elapsed = now - then;

    // if enough time has elapsed, draw the next frame

    if (elapsed > fpsInterval) {
        // Get ready for next frame by setting then=now, but also adjust for your
        // specified fpsInterval not being a multiple of RAF's interval (16.7ms)
        then = now - (elapsed % fpsInterval);

        // Put your drawing code here
        render();
    }
}

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        playPauseButton.textContent = "⏸";
        animate();
    } else {
        playPauseButton.textContent = "▶";
        cancelAnimationFrame(requestId);
        requestId = null;
    }
});

render();
startAnimating(10);