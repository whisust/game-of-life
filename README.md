<div align="center">

  <h1><code>WASM Game of Life</code></h1>

  <strong>Implementation of Conway's game of life using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

[tutorials]: https://rustwasm.github.io/docs/book/

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.


LAST STATE => https://rustwasm.github.io/docs/book/game-of-life/interactivity.html

# TODO
 - ~~replace array with fixed bit set~~
 - ~~add size input~~
 - ~~add reset button~~
 - ~~add spaceships on alt-click~~
 - add pulsar on shift-click
 - make it look nice