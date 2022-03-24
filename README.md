# An image resizer code by Rust and Angular

Live demo: [Open](https://h82258652.github.io/image-resizer/)

## Supported format:

* png
* gif

## How to build this:

1. Install Rust use rustup
2. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/)
3. Prepare your Angular environment
4. Run ```wasm-pack build```
5. Run ```cd web```
6. Run ```npm install```
7. Run ```ng build```(for me use ```ng build --base-href=/image-resizer/```)
8. Use http server like [http-server](https://www.npmjs.com/package/http-server) to host the files in dist folder

Have fun!