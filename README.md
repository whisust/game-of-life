<div align="center">

  <h1><code>WASM Game of Life</code></h1>

  <strong>Implementation of Conway's game of life using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

</div>

## About

This is a toy project, designed to learn how to use `wasm-pack` to create embeddable rust code and graphical html / js.

It has not other purpose than having some fun while playing with living cells.

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🛠️ Build the webpage with webpack and node

```
npm install
npm start
```


- `ctrl + <click>` to generate a pulsar
- `alt + <click>` to generate a glider
- `<click>` to toggle a cell



LAST STATE => https://rustwasm.github.io/docs/book/game-of-life/interactivity.html

## TODO
 - make it look nice
 - time profile js + wasm to optimize it
