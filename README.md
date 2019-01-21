# genby
Using rust to simulate an environment, visualized in js by compiling rust to wasm.

[Newest build](http://jonasbak.github.io/genby/)

## Setup
Requires `rustup`, `rustc`, `cargo`, `wasm-pack` and `yarn`

```
git clone git@github.com:JonasBak/genby.git
cd genby
```

To compile the rust sources into wasm and generate js api, run:
* `wasm-pack build`

To install all frontend dependencies:
* Run `yarn` in genby/www

To use the generated wasm package in js:
* Run `yarn link` in genby/pkg
* Run `yarn link genby` in genby/www

To be able to render the webgl (and run the frontend):
* `git clone git@github.com:JonasBak/webglfw.git`
*  Run `yarn link` in webglfw/pkg
*  Run `yarn link webglfw` in genby/www

## Run
`yarn start` in genby/www

## Rebuild
`wasm-pack build`
