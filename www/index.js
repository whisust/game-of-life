import {State} from "game-of-life";

const game = State.new(10, 5);
const pre = document.getElementById('game-of-life-canvas');
const infos = document.getElementById('infos');
const previousStates = new Set();
let isLooping = false;

let fpsInterval, startTime, now, then, elapsed;

// initialize the timer variables and start the animation

function startAnimating(fps) {
    fpsInterval = 1000 / fps;
    then = Date.now();
    startTime = then;
    animate();
}

function animate() {

    // request another frame

    requestAnimationFrame(animate);

    // calc elapsed time since last loop

    now = Date.now();
    elapsed = now - then;

    // if enough time has elapsed, draw the next frame

    if (elapsed > fpsInterval) {
        // Get ready for next frame by setting then=now, but also adjust for your
        // specified fpsInterval not being a multiple of RAF's interval (16.7ms)
        then = now - (elapsed % fpsInterval);

        // Put your drawing code here
        let newState = game.render();
        if (previousStates.has(newState)){
            isLooping = true;
        } else {
            previousStates.add(newState);
        }
        let infosString = `Grid ${game.width} x ${game.height} - Generation ${game.generation}`;
        if (isLooping) {
            infosString += `. Loop detected!`;
        }
        pre.textContent = newState;
        infos.textContent = infosString;
        game.next();
    }
}


startAnimating(5);