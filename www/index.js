import {State, Pattern} from "game-of-life";
import {memory} from "game-of-life/game_of_life_bg";

// Constants
const CELL_SIZE = 10;
const MARGIN = 1;
const CELL_WITH_MARGIN = CELL_SIZE + MARGIN;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Run variables
let game;
const infos = document.getElementById('infos');
const canvas = document.getElementById('game-of-life-canvas');
resetCanvas();
const ctx = canvas.getContext('2d');
const playPauseButton = document.getElementById('play-pause');

// Values for the animation function
let fpsInterval, startTime, now, then, elapsed;
let requestId;

// initialize the timer variables and start the animation
function getHeight() {
    return document.getElementById('game-height').value;
}

function getWidth() {
    return document.getElementById('game-width').value;
}

function resetCanvas() {
    canvas.height = CELL_WITH_MARGIN * getHeight() + 1;
    canvas.width = CELL_WITH_MARGIN * getWidth() + 1;
}

function startAnimating(fps) {
    fpsInterval = 1000 / fps;
    then = Date.now();
    startTime = then;
    animate();
}

const isPaused = () => {
    return requestId === null || requestId === undefined;
}

const getIndex = (row, column) => {
    return row * game.width + column;
}

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
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
    const cells = new Uint8Array(memory.buffer, cellsPointer, game.width * game.height / 8);

    ctx.beginPath();

    for (let row = 0; row < game.height; row++) {
        for (let col = 0; col < game.width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR;
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

const setPlayPauseButton = () => {
    playPauseButton.textContent = isPaused() ? "▶" : "⏸";
}

const draw = () => {
    drawGrid();
    drawCells();
    infos.textContent = `Grid ${game.width} x ${game.height} - Generation ${game.generation}`;
}
const render = () => {
    game.next();
    draw();
}

function init() {
    let width = getWidth();
    let height = getHeight();
    game = State.new(width, height);
    resetCanvas();
    draw();
    setPlayPauseButton();
}

function reset() {
    cancelAnimationFrame(requestId);
    requestId = null;
    init();
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
        render();
    }
}

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        playPauseButton.textContent = "⏸";
        startAnimating(20);
    } else {
        playPauseButton.textContent = "▶";
        cancelAnimationFrame(requestId);
        requestId = null;
    }
});

document.getElementById("reset").addEventListener("click", event => {
    reset();
})

document.getElementById("change-dimensions").addEventListener("click", event => {
    reset();
})

canvas.addEventListener("click", event => {
    const rect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;

    const canvasLeft = (event.clientX - rect.left) * scaleX;
    const canvasTop = (event.clientY - rect.top) * scaleY;

    const row = Math.min(Math.floor(canvasLeft / CELL_WITH_MARGIN), game.width - 1);
    const col = Math.min(Math.floor(canvasTop / CELL_WITH_MARGIN), game.height - 1);

    if (event.altKey) {
        game.add_pattern(row, col, Pattern.Glider);
    } else if (event.shiftKey) {
        game.add_pattern(row, col, Pattern.Pulsar);
    } else {
        game.toggle_cell(row, col);
    }

    draw();
})

init();